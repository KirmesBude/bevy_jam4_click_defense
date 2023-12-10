use std::time::Duration;

use bevy::prelude::*;

use crate::{castle::SpawnQueue, GameState};

pub struct UpgradePlugin;

/// This plugin handles attributes related stuff like health
/// Attribure logic is only active during the State `GameState::Playing`
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_cooldown_reduction.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Component)]
pub struct SpawnCooldownReduction {
    level: usize,
}

impl SpawnCooldownReduction {
    const BASE_VALUE: f32 = 0.05;
    const MAX_LEVEL: usize = 10;

    pub fn level_up(&mut self) -> bool {
        if self.level < Self::MAX_LEVEL {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> f32 {
        (self.level as f32) * Self::BASE_VALUE
    }

    pub fn cost(&self) -> usize {
        (self.level + 1) * 5
    }

    pub fn level(&self) -> usize {
        self.level
    }
}

#[derive(Debug, Default, Component)]
pub struct SoldierTechtree {}

fn spawn_cooldown_reduction(
    mut query: Query<(&SpawnCooldownReduction, &mut SpawnQueue), Changed<SpawnCooldownReduction>>,
) {
    for (spawn_cooldown_reduction, mut spawn_queue) in &mut query {
        spawn_queue.timer.set_duration(Duration::from_secs_f32(
            2.0 * (1.0 - spawn_cooldown_reduction.get()),
        ));
    }
}

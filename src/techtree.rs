use std::time::Duration;

use bevy::prelude::*;

use crate::{castle::SpawnQueue, GameState};

pub struct TechtreePlugin;

/// This plugin handles attributes related stuff like health
/// Attribure logic is only active during the State `GameState::Playing`
impl Plugin for TechtreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_cooldown_reduction.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Default, Component)]
pub struct SpawnCooldownReduction {
    pub level: usize,
}

impl SpawnCooldownReduction {
    const BASE_VALUE: f32 = 0.05;

    pub fn get(&self) -> f32 {
        (self.level as f32) * Self::BASE_VALUE
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

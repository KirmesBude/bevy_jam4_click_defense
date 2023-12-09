use bevy::prelude::*;

use crate::{
    castle::{EnemyCastle, SpawnQueue},
    units::UnitKind,
    GameState,
};
pub struct SpawnerPlugin;

// This plugin is responsible to control the game audio
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>()
            .add_systems(Update, tick_wave_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Resource)]
struct Wave {
    level: u32,
    timer: Timer,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            level: 1,
            timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Default, Component)]
struct Spawner;

fn tick_wave_timer(
    time: Res<Time>,
    mut wave: ResMut<Wave>,
    enemy_castle: Res<EnemyCastle>,
    mut spawn_queue: Query<&mut SpawnQueue>,
) {
    if let Some(entity) = enemy_castle.0 {
        if let Ok(mut spawn_queue) = spawn_queue.get_mut(entity) {
            if wave.timer.tick(time.delta()).just_finished() {
                (0..2 * wave.level).for_each(|_| {
                    spawn_queue.units.push_back(UnitKind::Soldier);
                });

                wave.level += 1;
            }
        }
    }
}

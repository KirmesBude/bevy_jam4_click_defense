use bevy::prelude::*;

use crate::{
    castle::{EnemyCastle, SpawnQueue},
    units::{
        upgrade::{AttackCooldownUpgrade, ShieldUpgrade},
        UnitKind,
    },
    GameState,
};

use super::upgrade::SpawnCooldownReduction;
pub struct SpawnerPlugin;

// This plugin is responsible to control the game audio
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>()
            .add_systems(Update, tick_wave_timer.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Resource)]
pub struct Wave {
    pub level: u32,
    pub timer: Timer,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            level: 1,
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Default, Component)]
struct Spawner;

fn tick_wave_timer(
    time: Res<Time>,
    mut wave: ResMut<Wave>,
    enemy_castle: Res<EnemyCastle>,
    mut spawn_queue: Query<(
        &mut SpawnQueue,
        &mut SpawnCooldownReduction,
        &mut ShieldUpgrade,
        &mut AttackCooldownUpgrade,
    )>,
) {
    if let Some(entity) = enemy_castle.0 {
        if let Ok((mut spawn_queue, mut spawn_cooldown, mut shield, mut attack_speed)) =
            spawn_queue.get_mut(entity)
        {
            if spawn_queue.units.is_empty() && wave.timer.tick(time.delta()).just_finished() {
                (0..30 * wave.level).for_each(|_| {
                    spawn_queue.units.push_back(UnitKind::Soldier);
                });

                wave.level += 1;

                if wave.level % 2 == 0 {
                    spawn_cooldown.level_up();
                    spawn_cooldown.level_up();
                    shield.level_up();
                    shield.level_up();
                    attack_speed.level_up();
                    attack_speed.level_up();
                }
            }
        }
    }
}

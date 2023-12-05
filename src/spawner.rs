use bevy::prelude::*;
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use rand_core::RngCore;

use crate::{actions::SpawnEnemy, GameState};
pub struct SpawnerPlugin;

// This plugin is responsible to control the game audio
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>()
            .add_systems(OnEnter(GameState::Playing), create_spawner)
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
    spawners: Query<&GlobalTransform, With<Spawner>>,
    mut spawnenemy_evw: EventWriter<SpawnEnemy>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    if wave.timer.tick(time.delta()).just_finished() {
        for transform in &spawners {
            (0..wave.level).for_each(|_| {
                let offset = Vec2::new(
                    (rng.next_u32() % 30) as f32 - 15.0,
                    (rng.next_u32() % 30) as f32 - 15.0,
                )
                .extend(0.0);
                spawnenemy_evw.send(SpawnEnemy {
                    translation: transform.translation() + offset,
                });
            });
        }

        wave.level += 1;
    }
}

#[derive(Debug, Default, Bundle)]
struct SpawnerBundle {
    spatial: SpatialBundle,
    spawner: Spawner,
}

fn create_spawner(mut commands: Commands) {
    let positions = vec![
        Vec2::new(-800.0, 50.0),
        Vec2::new(0.0, 500.0),
        Vec2::new(800.0, 50.0),
    ];
    for position in &positions {
        commands.spawn(SpawnerBundle {
            spatial: SpatialBundle {
                transform: Transform::from_translation(position.extend(0.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

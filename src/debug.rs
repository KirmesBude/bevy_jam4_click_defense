use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_xpbd_2d::components::{Collider, CollisionLayers};

use crate::{
    attributes::{ApplyHealthDelta, Health},
    behaviour::{Behaviour, DefaultBehaviour, EnemyFinderBundle},
    castle::AllyCastle,
    hit_detection::{HitBox, HitBoxBundle, HitBoxKind, HurtBoxBundle},
    loading::TextureAssets,
    physics::PhysicsCollisionBundle,
    units::{AttackCooldown, Faction, UnitKind},
    GameState,
};
use std::fmt::Debug;

pub struct DebugPlugin;

/// This plugin handles debug related stuff
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemy>()
            .add_event::<QueueEnemyUnit>()
            .add_systems(
                Update,
                (
                    debug_events::<ApplyHealthDelta>,
                    emit_spawn_action_mouse,
                    emit_queue_enemy_unit,
                    spawn_enemy,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn debug_events<E>(mut evr: EventReader<E>)
where
    E: Event + Debug,
{
    for ev in evr.read() {
        debug!("{:?}", ev);
    }
}

fn viewport_to_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}

#[derive(Debug, Event)]
pub struct SpawnEnemy {
    pub translation: Vec3,
}

fn emit_spawn_action_mouse(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut spawnenemy_evw: EventWriter<SpawnEnemy>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Pressed {
            let window = windows.get(ev.window).unwrap();
            let (camera, camera_transform) = cameras.get_single().unwrap();
            let world_position =
                viewport_to_world_position(window, camera, camera_transform).unwrap();
            info!("{}", world_position);

            if ev.button == MouseButton::Middle {
                spawnenemy_evw.send(SpawnEnemy {
                    translation: world_position.extend(0.0),
                });
            }
        }
    }
}

#[derive(Debug, Event)]
pub struct QueueEnemyUnit {
    pub kind: UnitKind,
}

fn emit_queue_enemy_unit(
    mut keyboard_evr: EventReader<KeyboardInput>,
    mut queueunit_evw: EventWriter<QueueEnemyUnit>,
) {
    for ev in keyboard_evr.read() {
        if let Some(KeyCode::Space) = ev.key_code {
            if ev.state == ButtonState::Pressed {
                queueunit_evw.send(QueueEnemyUnit {
                    kind: UnitKind::Soldier,
                });
            }
        }
    }
}

fn spawn_unit(
    commands: &mut Commands,
    faction: Faction,
    translation: Vec3,
    textures: &Res<TextureAssets>,
    behaviour: Behaviour,
    default_behaviour: Option<DefaultBehaviour>,
) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            texture: textures.enemy_soldier.clone(),
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(PhysicsCollisionBundle {
            collider: Collider::ball(10.0),
            ..Default::default()
        })
        .insert(behaviour)
        .insert(Health::new(100.0))
        .with_children(|children| {
            children.spawn(HurtBoxBundle {
                collider: Collider::ball(9.0),
                collisionlayers: CollisionLayers::new(
                    [faction.hurt_layer()],
                    [faction.opposite().hit_layer()],
                ),
                ..Default::default()
            });
            children
                .spawn(HitBoxBundle {
                    hitbox: HitBox {
                        damage: 10.0,
                        kind: HitBoxKind::Once(vec![]),
                    },
                    collider: Collider::ball(12.0),
                    collisionlayers: CollisionLayers::new(
                        [faction.hit_layer()],
                        [faction.opposite().hurt_layer()],
                    ),
                    ..Default::default()
                })
                .insert(AttackCooldown {
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                });
            children.spawn(EnemyFinderBundle {
                collider: Collider::ball(60.0),
                collisionlayers: CollisionLayers::new(
                    [faction.hit_layer()],
                    [faction.opposite().hurt_layer()],
                ),
                ..Default::default()
            });
        })
        .id();

    if let Some(default_behaviour) = default_behaviour {
        commands.entity(entity).insert(default_behaviour);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawnenemy_evr: EventReader<SpawnEnemy>,
    textures: Res<TextureAssets>,
    castle: Res<AllyCastle>,
) {
    for ev in spawnenemy_evr.read() {
        println!("spawn enemy");
        spawn_unit(
            &mut commands,
            Faction::Enemy,
            ev.translation,
            &textures,
            Behaviour::MoveAndAttack(castle.0.unwrap()),
            Some(DefaultBehaviour(Behaviour::MoveAndAttack(
                castle.0.unwrap(),
            ))),
        );
    }
}

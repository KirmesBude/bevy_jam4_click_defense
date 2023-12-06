use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, CollisionLayers},
    prelude::PhysicsLayer,
};

use crate::{
    actions::{QueueUnit, SpawnAlly, SpawnEnemy},
    attributes::Health,
    behaviour::{Behaviour, DefaultBehaviour, EnemyFinderBundle},
    castle::AllyCastle,
    hit_detection::{HitBox, HitBoxBundle, HitBoxKind, HurtBoxBundle},
    loading::TextureAssets,
    physics::{PhysicsCollisionBundle, SensorLayers},
    GameState,
};

pub struct UnitPluging;

/// This plugin handles unit related stuff
/// Unit logic is only active during the State `GameState::Playing`
impl Plugin for UnitPluging {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_ally,
                spawn_enemy,
                advance_attack_cooldown_timer,
                spawn_from_queue,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component, Clone, Copy, PhysicsLayer)]
pub enum Faction {
    Ally,
    Enemy,
}

impl Faction {
    pub fn color(&self) -> Color {
        match self {
            Self::Ally => Color::BLUE,
            Self::Enemy => Color::RED,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Ally => Self::Enemy,
            Self::Enemy => Self::Ally,
        }
    }

    pub fn hurt_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHurt,
            Faction::Enemy => SensorLayers::EnemyHurt,
        }
    }

    pub fn hit_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHit,
            Faction::Enemy => SensorLayers::EnemyHit,
        }
    }
}

pub fn spawn_unit(
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
                color: faction.color(),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            texture: textures.bevy.clone(),
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

pub fn spawn_enemy(
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

pub fn spawn_ally(
    mut commands: Commands,
    mut spawnally_evr: EventReader<SpawnAlly>,
    textures: Res<TextureAssets>,
) {
    for ev in spawnally_evr.read() {
        spawn_unit(
            &mut commands,
            Faction::Ally,
            ev.translation,
            &textures,
            Behaviour::default(),
            None,
        );
    }
}

#[derive(Debug, Component)]
pub struct AttackCooldown {
    timer: Timer,
}

fn advance_attack_cooldown_timer(
    time: Res<Time>,
    mut cooldowns: Query<(&mut AttackCooldown, &mut HitBox)>,
) {
    for (mut cooldown, mut hitbox) in &mut cooldowns {
        if cooldown.timer.tick(time.delta()).just_finished() {
            hitbox.clear();
        }
    }
}

fn spawn_from_queue(
    ally_castle: Res<AllyCastle>,
    transforms: Query<&GlobalTransform>,
    mut queueunit_evr: EventReader<QueueUnit>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(transform) = transforms.get(entity) {
            let translation = transform.translation();
            for ev in queueunit_evr.read() {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Faction::Ally.color(),
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..Default::default()
                        },
                        texture: textures.bevy.clone(),
                        transform: Transform::from_translation(
                            translation + Vec3::new(128.0, 0.0, 0.0),
                        ),
                        ..Default::default()
                    })
                    .insert(PhysicsCollisionBundle {
                        collider: Collider::ball(10.0),
                        ..Default::default()
                    })
                    .insert(Behaviour::MoveToPoint(Vec2::new(1280.0, 0.0)))
                    .insert(Health::new(100.0))
                    .with_children(|children| {
                        children.spawn(HurtBoxBundle {
                            collider: Collider::ball(9.0),
                            collisionlayers: CollisionLayers::new(
                                [Faction::Ally.hurt_layer()],
                                [Faction::Ally.opposite().hit_layer()],
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
                                    [Faction::Ally.hit_layer()],
                                    [Faction::Ally.opposite().hurt_layer()],
                                ),
                                ..Default::default()
                            })
                            .insert(AttackCooldown {
                                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                            });
                        children.spawn(EnemyFinderBundle {
                            collider: Collider::ball(60.0),
                            collisionlayers: CollisionLayers::new(
                                [Faction::Ally.hit_layer()],
                                [Faction::Ally.opposite().hurt_layer()],
                            ),
                            ..Default::default()
                        });
                    });
            }
        }
    }
}

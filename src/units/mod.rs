use bevy::prelude::*;
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use bevy_xpbd_2d::components::{Collider, CollidingEntities, CollisionLayers, Sensor};
use rand_core::RngCore;

use crate::{
    actions::{SpawnAlly, SpawnEnemy},
    attributes::Health,
    behaviour::{Behaviour, DefaultBehaviour, EnemyFinderBundle},
    castle::{AllyCastle, EnemyCastle, SpawnUnit},
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
                spawn_unit_from_event,
                spawn_protection,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component, Clone, Copy)]
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

fn spawn_unit_from_event(
    mut spawnunit_evr: EventReader<SpawnUnit>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    for ev in spawnunit_evr.read() {
        if let Ok(transform) = transforms.get(ev.origin) {
            let translation = transform.translation();

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: ev.faction.color(),
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    },
                    texture: textures.bevy.clone(),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                })
                .insert(Behaviour::MoveToPoint(Vec2::new(
                    0.0,
                    (rng.next_u32() % 720) as f32 - 360.0,
                )))
                .insert(Health::new(100.0))
                .insert(PhysicsCollisionBundle {
                    collider: Collider::ball(10.0),
                    ..Default::default()
                })
                .insert(Sensor)
                .insert(SpawnProtection::default())
                .insert(ev.faction);
        }
    }
}

/* Spawn with this instead */
#[derive(Debug, Component)]
pub struct SpawnProtection(Timer);

impl Default for SpawnProtection {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

fn spawn_protection(
    mut query: Query<(
        Entity,
        &CollidingEntities,
        &Faction,
        &mut SpawnProtection,
        &mut Behaviour,
    )>,
    mut commands: Commands,
    time: Res<Time>,
    ally_castle: Res<AllyCastle>,
    enemy_castle: Res<EnemyCastle>,
) {
    for (entity, colliding_entities, faction, mut spawn_protection, mut behaviour) in &mut query {
        if spawn_protection.0.tick(time.delta()).finished() && colliding_entities.is_empty() {
            *behaviour = match faction {
                Faction::Ally => Behaviour::MoveAndAttack(enemy_castle.0.unwrap()),
                Faction::Enemy => Behaviour::MoveAndAttack(ally_castle.0.unwrap()),
            };

            /* TODO: A bit janky if units with spawn protection overlap */
            /* Now we can remove Sensor and SpawnProtection */
            /* And add hurt and hitboxes and enemyfinder */
            commands.entity(entity).remove::<Sensor>().remove::<SpawnProtection>()
            .insert(DefaultBehaviour(behaviour.clone()))
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
                                collider: Collider::ball(12.0), /* TODO: Get these numbers from somewhere */
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
                    });
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnitKind {
    Soldier,
}

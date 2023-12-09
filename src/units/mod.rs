pub mod behaviour;

use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollidingEntities, CollisionLayers, Sensor};

use crate::{
    castle::{AllyCastle, EnemyCastle, SpawnUnit},
    common::attributes::Health,
    common::Faction,
    loading::TextureAssets,
    physics::hit_detection::{HitBox, HitBoxBundle, HitBoxKind, HurtBoxBundle},
    physics::PhysicsCollisionBundle,
    GameState,
};

use self::behaviour::{Behaviour, BehaviourPlugin, DefaultBehaviour, EnemyFinderBundle};

pub struct UnitPluging;

/// This plugin handles unit related stuff
/// Unit logic is only active during the State `GameState::Playing`
impl Plugin for UnitPluging {
    fn build(&self, app: &mut App) {
        app.add_plugins(BehaviourPlugin).add_systems(
            Update,
            (
                advance_attack_cooldown_timer,
                spawn_unit_from_event,
                spawn_protection,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component)]
pub struct AttackCooldown {
    pub timer: Timer,
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
    mut y: Local<f32>,
) {
    for ev in spawnunit_evr.read() {
        if let Ok(transform) = transforms.get(ev.origin) {
            let translation = transform.translation();
            *y = (*y + 60.0) % 360.0;

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(20.0, 20.0)),
                        ..Default::default()
                    },
                    texture: ev.kind.texture(&ev.faction, &textures),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                })
                .insert(Behaviour::MoveToPoint(Vec2::new(0.0, *y - 180.0)))
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

impl UnitKind {
    pub fn cost(&self) -> usize {
        match self {
            UnitKind::Soldier => 1,
        }
    }
}

impl UnitKind {
    pub fn texture(&self, faction: &Faction, texture_assets: &TextureAssets) -> Handle<Image> {
        match self {
            UnitKind::Soldier => match faction {
                Faction::Ally => texture_assets.ally_soldier.clone(),
                Faction::Enemy => texture_assets.enemy_soldier.clone(),
            },
        }
    }
}

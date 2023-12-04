use std::ops::Sub;

use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, CollisionLayers, LinearVelocity, Sensor, CollidingEntities, ColliderParent},
    prelude::PhysicsLayer, plugins::collision::contact_reporting::Collision,
};

use crate::{
    actions::{SpawnAlly, SpawnEnemy},
    attributes::{Health, ApplyHealthDelta},
    castle::MainCastle,
    loading::TextureAssets,
    GameState, physics::{PhysicsCollisionBundle, SensorLayers},
};

pub struct UnitPluging;

/// This plugin handles unit related stuff
/// Unit logic is only active during the State `GameState::Playing`
impl Plugin for UnitPluging {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(
            Update,
            (spawn_ally, spawn_enemy, move_towards, hit_detection).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component, Clone, Copy, PhysicsLayer)]
enum Faction {
    Ally,
    Enemy,
}

impl Faction {
    fn color(&self) -> Color {
        match self {
            Self::Ally => Color::BLUE,
            Self::Enemy => Color::RED,
        } 
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Ally => Self::Enemy,
            Self::Enemy => Self::Ally,
        }
    }

    fn hurt_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHurt,
            Faction::Enemy => SensorLayers::EnemyHurt,
        }
    }

    fn hit_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHit,
            Faction::Enemy => SensorLayers::EnemyHit,
        }
    }
}

fn spawn_unit(
    commands: &mut Commands,
    faction: Faction,
    translation: Vec3,
    textures: &Res<TextureAssets>,
    entity: Option<Entity>,
) {
    commands
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
        .insert(PhysicsCollisionBundle { collider: Collider::ball(10.0), ..Default::default()})
        .insert(MoveTowards { entity })
        .with_children(|children| {
            match faction {
                Faction::Ally => children.spawn(HurtBoxBundle {
                    collider: Collider::ball(9.0),
                    collisionlayers: CollisionLayers::new([faction.hurt_layer()], [faction.opposite().hit_layer()]),
                    ..Default::default()
                }),
                Faction::Enemy => children.spawn(HitBoxBundle {
                    collider: Collider::ball(12.0),
                    collisionlayers: CollisionLayers::new([faction.hit_layer()], [faction.opposite().hurt_layer()]),
                    ..Default::default()
                }),
            };
        });
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut spawnenemy_evr: EventReader<SpawnEnemy>,
    textures: Res<TextureAssets>,
    castle: Query<Entity, With<MainCastle>>,
) {
    for ev in spawnenemy_evr.read() {
        spawn_unit(
            &mut commands,
            Faction::Enemy,
            ev.translation,
            &textures,
            Some(castle.single()),
        );
    }
}

pub fn spawn_ally(
    mut commands: Commands,
    mut spawnally_evr: EventReader<SpawnAlly>,
    textures: Res<TextureAssets>,
    castle: Query<Entity, With<MainCastle>>,
) {
    for ev in spawnally_evr.read() {
        spawn_unit(
            &mut commands,
            Faction::Ally,
            ev.translation,
            &textures,
            Some(castle.single()),
        );
    }
}

#[derive(Debug, Component)]
pub struct MoveTowards {
    entity: Option<Entity>,
}

fn move_towards(
    mut query: Query<(Entity, &mut LinearVelocity, &MoveTowards)>,
    transforms: Query<&GlobalTransform>,
) {
    for (source_entity, mut velocity, move_towards) in &mut query {
        let source_transform = transforms.get(source_entity).unwrap();
        let target_transform = transforms.get(move_towards.entity.unwrap()).unwrap();

        let vector = target_transform
            .translation()
            .sub(source_transform.translation())
            .normalize()
            .truncate()
            * 50.0;

        velocity.0 = vector;
    }
}

#[derive(Debug, Component)]
pub struct AttackCooldown {
    timer: Timer,
}

fn advance_attack_cooldown_timer(
    time: Res<Time>,
    mut cooldowns: Query<(&mut AttackCooldown, &GlobalTransform)>,
    mut attackevent_evw: EventWriter<AttackEvent>,
) {
    for (mut cooldown, transform) in &mut cooldowns {
        if cooldown.timer.tick(time.delta()).just_finished() {
            attackevent_evw.send(AttackEvent { position: transform.translation().truncate(), duration: 1.0, damage: 5.0 })
        }
    }
}

#[derive(Debug, Event)]
pub struct AttackEvent {
    position: Vec2,
    duration: f32,
    damage: f32,
}

fn spawn_attack(
    mut commands: Commands,
    mut attackevent_evr: EventReader<AttackEvent>,
) {
    for attackevent in attackevent_evr.read() {
        commands.spawn(
            AttackBundle {
                attack: Attack(attackevent.damage),
                alive_timer: AliveTimer(Timer::from_seconds(attackevent.duration, TimerMode::Once)),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(attackevent.position.extend(0.0)),
                    ..Default::default()
                },
                collider: Collider::ball(15.0),
                collision_layers: CollisionLayers::new([SensorLayers::EnemyHit], [SensorLayers::AllyHurt]),
                ..Default::default()
            }
        );
    }
}

#[derive(Debug, Default, Component, Deref, DerefMut)]
pub struct Attack(f32);

#[derive(Debug, Default, Bundle)]
pub struct AttackBundle {
    attack: Attack,
    alive_timer: AliveTimer,
    spatial: SpatialBundle,
    collider: Collider,
    sensor: Sensor,
    collision_layers: CollisionLayers,
}

#[derive(Debug, Default, Component, Deref, DerefMut)]
pub struct AliveTimer(Timer);

fn despawn_from_alive_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut alive_timers: Query<(Entity, &mut AliveTimer)>,
) {
    for (entity, mut alive_timer) in &mut alive_timers {
        if alive_timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn damage_stuff(
    mut collision_event_reader: EventReader<Collision>,
    mut applyhealthdelta_evw: EventWriter<ApplyHealthDelta>,
    attacks: Query<&Attack>,
    healths: Query<&Health>,
    time: Res<Time>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        eprintln!("{:?}", contacts);
        let delta = 10.0 * time.delta_seconds();
        if attacks.contains(contacts.entity1) && healths.contains(contacts.entity2) {
            eprintln!("lol1");
            applyhealthdelta_evw.send(ApplyHealthDelta { entity: contacts.entity2, delta });
        } else if attacks.contains(contacts.entity2) && healths.contains(contacts.entity1) {
            eprintln!("lol2");
            applyhealthdelta_evw.send(ApplyHealthDelta { entity: contacts.entity1, delta });
        }
    }
}

#[derive(Debug, Default, Component)]
pub enum HitBox {
    Once(Vec<Entity>),
    #[default]
    Persistent,
}

#[derive(Debug, Default, Bundle)]
pub struct HitBoxBundle {
    hitbox: HitBox,
    collider: Collider,
    sensor: Sensor,
    collisionlayers: CollisionLayers,
}

#[derive(Debug, Default, Component)]
pub struct HurtBox;

#[derive(Debug, Default, Bundle)]
pub struct HurtBoxBundle {
    hurtbox: HurtBox,
    collider: Collider,
    sensor: Sensor,
    collisionlayers: CollisionLayers,
}

fn hit_detection(
    hit_boxes: Query<(&ColliderParent, &CollidingEntities), With<HitBox>>,
    hurt_boxes: Query<(), With<HurtBox>>,
) {
    for (parent, colliding_entities) in &hit_boxes {
        let colliding_entities: Vec<&Entity> = colliding_entities.iter().filter(|entity| hurt_boxes.contains(**entity)).collect();
        if !colliding_entities.is_empty() {
            println!(
                "{:?} is colliding with the following entities: {:?}",
                parent.get(),
                colliding_entities
            );
        }
    }
}

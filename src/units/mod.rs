use std::ops::Sub;

use bevy::prelude::*;
use bevy_xpbd_2d::{
    components::{Collider, CollisionLayers, LinearVelocity},
    prelude::PhysicsLayer,
};

use crate::{
    actions::{SpawnAlly, SpawnEnemy},
    attributes::Health,
    castle::MainCastle,
    hit_detection::{HitBoxBundle, HurtBoxBundle, HitBox, HitBoxKind},
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
            (spawn_ally, spawn_enemy, move_towards, advance_attack_cooldown_timer).run_if(in_state(GameState::Playing)),
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
        .insert(PhysicsCollisionBundle {
            collider: Collider::ball(10.0),
            ..Default::default()
        })
        .insert(Health::new(100.0))
        .insert(MoveTowards { entity })
        .with_children(|children| {
            children.spawn(HurtBoxBundle {
                collider: Collider::ball(9.0),
                collisionlayers: CollisionLayers::new(
                    [faction.hurt_layer()],
                    [faction.opposite().hit_layer()],
                ),
                ..Default::default()
            });
            children.spawn(HitBoxBundle {
                hitbox: HitBox { damage: 10.0, kind: HitBoxKind::Once(vec![]) },
                collider: Collider::ball(12.0),
                collisionlayers: CollisionLayers::new(
                    [faction.hit_layer()],
                    [faction.opposite().hurt_layer()],
                ),
                ..Default::default()
            }).insert(AttackCooldown { timer: Timer::from_seconds(1.0, TimerMode::Repeating)});
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
            * 30.0;

        velocity.0 = vector;
    }
}

/* TODO: Actually place this on HitBox */
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

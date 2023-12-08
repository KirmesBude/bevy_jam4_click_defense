use std::collections::VecDeque;

use crate::actions::QueueAllyUnit;
use crate::attributes::{Health, Immortal};
use crate::hit_detection::HurtBoxBundle;
use crate::loading::TextureAssets;
use crate::physics::PhysicsCollisionBundle;
use crate::techtree::SpawnCooldownReduction;
use crate::units::{Faction, UnitKind};
use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_2d::components::{Collider, CollisionLayers, RigidBody};

pub struct CastlePlugin;

#[derive(Component)]
pub struct Castle;

#[derive(Debug, Default, Resource, Deref)]
pub struct AllyCastle(pub Option<Entity>);

#[derive(Debug, Default, Resource, Deref)]
pub struct EnemyCastle(pub Option<Entity>);

#[derive(Component)]
pub struct AllyCastleHealthUI;

/// This plugin handles castle related stuff like health ui
/// Castle logic is only active during the State `GameState::Playing`
impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllyCastle>()
            .init_resource::<EnemyCastle>()
            .init_resource::<UnitPoints>()
            .add_event::<SpawnUnit>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    spawn_ally_castle,
                    spawn_enemy_castle,
                    spawn_health_ui,
                    init_unit_points,
                ),
            )
            .add_systems(
                Update,
                (
                    update_health,
                    spawn_queue,
                    process_queue_ally_unit,
                    game_over,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_ally_castle(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut ally_castle: ResMut<AllyCastle>,
) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: textures.castle.clone(),
            transform: Transform::from_translation(Vec3::new(-1280.0 / 2.0, 0., 1.)),
            ..Default::default()
        })
        .insert(Faction::Ally)
        .insert(Castle)
        .insert(Immortal)
        .insert(Health::new(1000.0))
        .insert(PhysicsCollisionBundle {
            rigid_body: RigidBody::Static,
            collider: Collider::ball(128.0),
            ..Default::default()
        })
        .insert(SpawnQueue::default())
        .insert(SpawnCooldownReduction::default())
        .with_children(|children| {
            children.spawn(HurtBoxBundle {
                collider: Collider::ball(127.0),
                collisionlayers: CollisionLayers::new(
                    [Faction::Ally.hurt_layer()],
                    [Faction::Ally.opposite().hit_layer()],
                ),
                ..Default::default()
            });
        })
        .id();

    ally_castle.0 = Some(entity);
}

fn spawn_enemy_castle(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut enemy_castle: ResMut<EnemyCastle>,
) {
    let entity = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            },
            texture: textures.castle.clone(),
            transform: Transform::from_translation(Vec3::new(1280.0 / 2.0, 0., 1.)),
            ..Default::default()
        })
        .insert(Faction::Enemy)
        .insert(Castle)
        .insert(Immortal)
        .insert(Health::new(1000.0))
        .insert(PhysicsCollisionBundle {
            rigid_body: RigidBody::Static,
            collider: Collider::ball(128.0),
            ..Default::default()
        })
        .insert(SpawnQueue::default())
        .insert(SpawnCooldownReduction::default())
        .with_children(|children| {
            children.spawn(HurtBoxBundle {
                collider: Collider::ball(127.0),
                collisionlayers: CollisionLayers::new(
                    [Faction::Enemy.hurt_layer()],
                    [Faction::Ally.opposite().hit_layer()],
                ),
                ..Default::default()
            });
        })
        .id();

    enemy_castle.0 = Some(entity);
}

fn spawn_health_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "CastleHealth",
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
        AllyCastleHealthUI,
    ));
}

fn update_health(
    ally_castle: Res<AllyCastle>,
    castle_healths: Query<&Health, With<Castle>>,
    mut castle_health_ui: Query<&mut Text, With<AllyCastleHealthUI>>,
) {
    if let Some(entity) = ally_castle.0 {
        let castle_health = castle_healths.get(entity).unwrap();
        let mut castle_health_ui = castle_health_ui.single_mut();

        castle_health_ui.sections[0].value = format!("{}", castle_health);
    }
}

#[derive(Debug, Component)]
pub struct SpawnQueue {
    pub timer: Timer,
    pub units: VecDeque<UnitKind>,
}

impl Default for SpawnQueue {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            units: Default::default(),
        }
    }
}

fn spawn_queue(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnQueue, &Faction)>,
    mut spawnunit_evw: EventWriter<SpawnUnit>,
) {
    for (origin, mut spawn_queue, faction) in &mut query {
        if !spawn_queue.units.is_empty() && spawn_queue.timer.tick(time.delta()).just_finished() {
            spawnunit_evw.send(SpawnUnit {
                origin,
                faction: *faction,
                kind: spawn_queue.units.pop_front().unwrap(),
            });
        }
    }
}

#[derive(Debug, Event)]
pub struct SpawnUnit {
    pub origin: Entity,
    pub faction: Faction,
    pub kind: UnitKind,
}

fn process_queue_ally_unit(
    mut queueallyunit_evr: EventReader<QueueAllyUnit>,
    mut spawn_queue: Query<&mut SpawnQueue>,
    ally_castle: Res<AllyCastle>,
    mut unit_points: ResMut<UnitPoints>,
) {
    for ev in queueallyunit_evr.read() {
        if unit_points.0 > 0 {
            if let Some(entity) = ally_castle.0 {
                if let Ok(mut spawn_queue) = spawn_queue.get_mut(entity) {
                    spawn_queue.units.push_back(ev.kind);
                    unit_points.0 -= 1;
                }
            }
        }
    }
}

fn game_over(
    health: Query<&Health>,
    ally_castle: Res<AllyCastle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(entity) = ally_castle.0 {
        if let Ok(health) = health.get(entity) {
            if health.current == 0.0 {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct UnitPoints(pub usize);

fn init_unit_points(mut unit_points: ResMut<UnitPoints>) {
    unit_points.0 = 5;
}

use std::ops::Sub;

use bevy::prelude::*;
use bevy_xpbd_2d::components::{RigidBody, LinearVelocity};

use crate::{
    actions::{SpawnAlly, SpawnEnemy},
    loading::TextureAssets,
    GameState, castle::MainCastle,
};

pub struct UnitPluging;

/// This plugin handles unit related stuff
/// Unit logic is only active during the State `GameState::Playing`
impl Plugin for UnitPluging {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_ally, spawn_enemy, move_towards).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component, Clone, Copy)]
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
        .insert(faction)
        .insert(RigidBody::Dynamic)
        .insert(MoveTowards {
            entity
        });
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut spawnenemy_evr: EventReader<SpawnEnemy>,
    textures: Res<TextureAssets>,
    castle: Query<Entity, With<MainCastle>>,
) {
    for ev in spawnenemy_evr.read() {
        spawn_unit(&mut commands, Faction::Enemy, ev.translation, &textures, Some(castle.single()));
    }
}

pub fn spawn_ally(
    mut commands: Commands,
    mut spawnally_evr: EventReader<SpawnAlly>,
    textures: Res<TextureAssets>,
    castle: Query<Entity, With<MainCastle>>,
) {
    for ev in spawnally_evr.read() {
        spawn_unit(&mut commands, Faction::Ally, ev.translation, &textures, Some(castle.single()));
    }
}

#[derive(Debug, Component)]
pub struct MoveTowards {
    entity: Option<Entity>
}

fn move_towards(
    mut query: Query<(Entity, &mut LinearVelocity, &MoveTowards)>,
    transforms: Query<&GlobalTransform>,
) {
    for (source_entity, mut velocity, move_towards) in &mut query {
        let source_transform = transforms.get(source_entity).unwrap();
        let target_transform = transforms.get(move_towards.entity.unwrap()).unwrap();

        let vector = target_transform.translation().sub(source_transform.translation()).normalize().truncate() * 50.0;

        velocity.0 = vector;
    }
}
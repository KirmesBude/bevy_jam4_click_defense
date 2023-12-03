use bevy::prelude::*;

use crate::{
    actions::{SpawnAlly, SpawnEnemy},
    loading::TextureAssets,
    GameState,
};

pub struct UnitPluging;

/// This plugin handles unit related stuff
/// Unit logic is only active during the State `GameState::Playing`
impl Plugin for UnitPluging {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_ally, spawn_enemy).run_if(in_state(GameState::Playing)),
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
        .insert(faction);
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut spawnenemy_evr: EventReader<SpawnEnemy>,
    textures: Res<TextureAssets>,
) {
    for ev in spawnenemy_evr.read() {
        spawn_unit(&mut commands, Faction::Enemy, ev.translation, &textures);
    }
}

pub fn spawn_ally(
    mut commands: Commands,
    mut spawnally_evr: EventReader<SpawnAlly>,
    textures: Res<TextureAssets>,
) {
    for ev in spawnally_evr.read() {
        spawn_unit(&mut commands, Faction::Ally, ev.translation, &textures);
    }
}

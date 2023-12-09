#![allow(clippy::type_complexity)]

mod attributes;
mod audio;
mod behaviour;
mod castle;
mod debug;
mod game_ui;
mod hit_detection;
mod loading;
mod menu;
mod physics;
mod spawner;
mod techtree;
mod units;

use crate::audio::InternalAudioPlugin;
use crate::castle::CastlePlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use attributes::AttributesPlugin;
use behaviour::BehaviourPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rand::{plugin::EntropyPlugin, prelude::ChaCha8Rng};
use debug::DebugPlugin;
use game_ui::GameUiPlugin;
use hit_detection::HitDetectionPlugin;
use physics::InternalPhysicsPlugin;
use spawner::SpawnerPlugin;
use techtree::TechtreePlugin;
use units::UnitPluging;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // GameOver
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            InternalAudioPlugin,
            CastlePlugin,
            UnitPluging,
            AttributesPlugin,
            InternalPhysicsPlugin,
            HitDetectionPlugin,
            SpawnerPlugin,
            EntropyPlugin::<ChaCha8Rng>::default(),
            BehaviourPlugin,
            GameUiPlugin,
            TechtreePlugin,
        ));
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                DebugPlugin,
            ));
        }
    }
}

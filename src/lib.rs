#![allow(clippy::type_complexity)]

mod actions;
mod attributes;
mod audio;
mod behaviour;
mod castle;
mod debug;
mod hit_detection;
mod loading;
mod menu;
mod physics;
mod spawner;
mod units;

use crate::actions::ActionsPlugin;
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
use hit_detection::HitDetectionPlugin;
use physics::InternalPhysicsPlugin;
use spawner::SpawnerPlugin;
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
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            CastlePlugin,
            UnitPluging,
            AttributesPlugin,
            DebugPlugin,
            InternalPhysicsPlugin,
            HitDetectionPlugin,
            SpawnerPlugin,
            EntropyPlugin::<ChaCha8Rng>::default(),
            BehaviourPlugin,
        ));
        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

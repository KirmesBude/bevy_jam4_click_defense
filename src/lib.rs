#![allow(clippy::type_complexity)]

mod audio;
mod castle;
mod common;
mod debug;
mod loading;
mod physics;
mod ui;
mod units;

use crate::audio::InternalAudioPlugin;
use crate::castle::CastlePlugin;
use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use common::CommonPlugin;
#[cfg(debug_assertions)]
use debug::DebugPlugin;
use physics::InternalPhysicsPlugin;
use ui::InternalUiPlugin;
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
            InternalUiPlugin,
            InternalAudioPlugin,
            CastlePlugin,
            UnitPluging,
            CommonPlugin,
            InternalPhysicsPlugin,
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

pub mod game_ui;
pub mod menu;

use bevy::prelude::*;

use self::{game_ui::GameUiPlugin, menu::MenuPlugin};

pub struct InternalUiPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, GameUiPlugin));
    }
}

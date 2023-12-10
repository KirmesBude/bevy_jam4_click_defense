use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Abstraction-Three-Red-Hearts-Go-_No-Vocal_.ogg")]
    pub go: Handle<AudioSource>,
    #[asset(path = "audio/Abstraction-Three-Red-Hearts-Rumble-at-the-Gates.ogg")]
    pub game: Handle<AudioSource>,
    #[asset(path = "audio/hit_005.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/castle.png")]
    pub castle: Handle<Image>,
    #[asset(path = "textures/ally_soldier.png")]
    pub ally_soldier: Handle<Image>,
    #[asset(path = "textures/enemy_soldier.png")]
    pub enemy_soldier: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/soldier_button.png")]
    pub soldier_button: Handle<Image>,
    #[asset(path = "textures/tech_castle_button.png")]
    pub tech_castle_button: Handle<Image>,
    #[asset(path = "textures/soldier_attackspeed.png")]
    pub soldier_attackspeed: Handle<Image>,
    #[asset(path = "textures/soldier_shield.png")]
    pub soldier_shield: Handle<Image>,
    #[asset(path = "textures/background.png")]
    pub background: Handle<Image>,
}

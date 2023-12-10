use crate::loading::AudioAssets;
use crate::GameState;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), start_go_audio)
            .add_systems(
                OnEnter(GameState::Playing),
                (stop_go_audio, start_game_audio).chain(),
            );
    }
}

#[derive(Component)]
struct GoAudio;

fn start_go_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, mut init: Local<bool>) {
    if !*init {
        commands.spawn((
            AudioBundle {
                source: audio_assets.go.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    ..Default::default()
                },
            },
            GoAudio,
        ));
        *init = true;
    }
}

fn stop_go_audio(mut commands: Commands, go_audio: Query<Entity, With<GoAudio>>) {
    for entity in &go_audio {
        commands.entity(entity).despawn_recursive();
    }
}

fn start_game_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.game.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..Default::default()
        },
    });
}

use std::time::Duration;

use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Menu), start_go_audio)
            .add_systems(
                OnEnter(GameState::Playing),
                (stop_go_audio, start_game_audio).chain(),
            );
    }
}

#[derive(Resource)]
struct GoAudio(Handle<AudioInstance>);

fn start_go_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.go.clone())
        .looped()
        .with_volume(1.0)
        .handle();

    commands.insert_resource(GoAudio(handle));
}

fn stop_go_audio(go_audio: Res<GoAudio>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    if let Some(instance) = audio_instances.get_mut(&go_audio.0) {
        instance.stop(AudioTween::linear(Duration::from_secs_f32(0.5)));
    }
}

fn start_game_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    audio
        .play(audio_assets.game.clone())
        .looped()
        .with_volume(1.0);
}

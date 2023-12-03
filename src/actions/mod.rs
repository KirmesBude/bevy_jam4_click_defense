use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::actions::game_control::viewport_to_world_position;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemy>()
            .add_event::<SpawnAlly>()
            .add_systems(
                Update,
                emit_spawn_action.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Debug, Event)]
pub struct SpawnEnemy {
    pub translation: Vec3,
}

#[derive(Debug, Event)]
pub struct SpawnAlly {
    pub translation: Vec3,
}

/* TODO: Touch Controls */
pub fn emit_spawn_action(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut spawnally_evw: EventWriter<SpawnAlly>,
    mut spawnenemy_evw: EventWriter<SpawnEnemy>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Pressed {
            let window = windows.get(ev.window).unwrap();
            let (camera, camera_transform) = cameras.get_single().unwrap();
            let world_position =
                viewport_to_world_position(window, camera, camera_transform).unwrap();
            info!("{}", world_position);

            match ev.button {
                MouseButton::Left => spawnally_evw.send(SpawnAlly {
                    translation: world_position.extend(0.0),
                }),
                MouseButton::Right => spawnenemy_evw.send(SpawnEnemy {
                    translation: world_position.extend(0.0),
                }),
                _ => (),
            }
        }
    }
}

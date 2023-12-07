use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::actions::game_control::viewport_to_world_position;
use crate::units::UnitKind;
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemy>()
            .add_event::<SpawnAlly>()
            .add_event::<QueueAllyUnit>()
            .add_systems(
                Update,
                (emit_spawn_action_mouse, emit_queue_ally_unit)
                    .run_if(in_state(GameState::Playing)),
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

pub fn emit_spawn_action_mouse(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut spawnally_evw: EventWriter<SpawnAlly>,
    mut spawnenemy_evw: EventWriter<SpawnEnemy>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for ev in mousebtn_evr.read() {
        if ev.state == ButtonState::Pressed {
            let window = windows.get(ev.window).unwrap();
            let (camera, camera_transform) = cameras.get_single().unwrap();
            let world_position =
                viewport_to_world_position(window, camera, camera_transform).unwrap();
            info!("{}", world_position);

            match ev.button {
                /*
                MouseButton::Left => spawnally_evw.send(SpawnAlly {
                    translation: world_position.extend(0.0),
                }),
                */
                MouseButton::Middle => spawnenemy_evw.send(SpawnEnemy {
                    translation: world_position.extend(0.0),
                }),
                _ => (),
            }
        }
    }
}

#[derive(Debug, Event)]
pub struct QueueAllyUnit {
    pub kind: UnitKind,
}

pub fn emit_queue_ally_unit(
    mut keyboard_evr: EventReader<KeyboardInput>,
    mut queueunit_evw: EventWriter<QueueAllyUnit>,
) {
    for ev in keyboard_evr.read() {
        if let Some(KeyCode::Space) = ev.key_code {
            if ev.state == ButtonState::Pressed {
                queueunit_evw.send(QueueAllyUnit {
                    kind: UnitKind::Soldier,
                });
            }
        }
    }
}

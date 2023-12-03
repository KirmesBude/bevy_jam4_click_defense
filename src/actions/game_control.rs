use bevy::prelude::*;

pub fn viewport_to_world_position(window: &Window, camera: &Camera, camera_transform: &GlobalTransform) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
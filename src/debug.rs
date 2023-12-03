use bevy::prelude::*;

use crate::attributes::ApplyHealthDelta;
use std::fmt::Debug;

pub struct DebugPlugin;

/// This plugin handles debug related stuff
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_events::<ApplyHealthDelta>);
    }
}

fn debug_events<E>(mut evr: EventReader<E>)
where
    E: Event + Debug,
{
    for ev in evr.read() {
        debug!("{:?}", ev);
    }
}

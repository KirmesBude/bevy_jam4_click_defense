use bevy::prelude::*;
use bevy_xpbd_2d::{prelude::PhysicsLayer, components::{CollisionLayers, Collider, RigidBody}, plugins::{PhysicsPlugins, PhysicsDebugPlugin}, resources::Gravity};

pub struct InternalPhysicsPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        )).insert_resource(Gravity::ZERO);
    }
}

#[derive(Debug)]
pub struct PhysicsCollisionLayer;

impl PhysicsLayer for PhysicsCollisionLayer {
    fn to_bits(&self) -> u32 {
        1<<31
    }

    fn all_bits() -> u32 {
        1<<31 /* TODO: Not sure */
    }
}

#[derive(Debug, Bundle)]
pub struct PhysicsCollisionBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_layer: CollisionLayers,
}

impl Default for PhysicsCollisionBundle {
    fn default() -> Self {
        Self { rigid_body: Default::default(), collider: Default::default(), collision_layer: CollisionLayers::new([PhysicsCollisionLayer], [PhysicsCollisionLayer]) }
    }
}

#[derive(Debug, PhysicsLayer)]
pub enum SensorLayers {
    AllyHurt,
    AllyHit,
    EnemyHurt,
    EnemyHit,
}


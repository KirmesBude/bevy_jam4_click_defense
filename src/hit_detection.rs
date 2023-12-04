use bevy::prelude::*;
use bevy_xpbd_2d::components::{
    Collider, ColliderParent, CollidingEntities, CollisionLayers, Sensor,
};

pub struct HitDetectionPlugin;

// This plugin is responsible to control the game audio
impl Plugin for HitDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hit_detection);
    }
}

#[derive(Debug, Default, Component)]
pub enum HitBox {
    Once(Vec<Entity>),
    #[default]
    Persistent,
}

#[derive(Debug, Default, Bundle)]
pub struct HitBoxBundle {
    pub hitbox: HitBox,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collisionlayers: CollisionLayers,
}

#[derive(Debug, Default, Component)]
pub struct HurtBox;

#[derive(Debug, Default, Bundle)]
pub struct HurtBoxBundle {
    pub hurtbox: HurtBox,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collisionlayers: CollisionLayers,
}

fn hit_detection(
    hit_boxes: Query<(&ColliderParent, &CollidingEntities), With<HitBox>>,
    hurt_boxes: Query<&ColliderParent, With<HurtBox>>,
) {
    for (parent, colliding_entities) in &hit_boxes {
        let colliding_entities: Vec<Entity> = colliding_entities
            .iter()
            .filter(|entity| hurt_boxes.contains(**entity))
            .map(|entity| hurt_boxes.get(*entity).unwrap().get())
            .collect();
        if !colliding_entities.is_empty() {
            println!(
                "{:?} is colliding with the following entities: {:?}",
                parent.get(),
                colliding_entities
            );
        }
    }
}

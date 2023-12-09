use bevy::prelude::*;
use bevy_xpbd_2d::components::{
    Collider, ColliderParent, CollidingEntities, CollisionLayers, Sensor,
};

use crate::common::attributes::ApplyHealthDelta;

pub struct HitDetectionPlugin;

// This plugin is responsible to control the game audio
impl Plugin for HitDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hit_detection);
    }
}

#[derive(Debug, Default, Component)]
pub struct HitBox {
    pub damage: f32,
    pub kind: HitBoxKind,
}

impl HitBox {
    pub fn clear(&mut self) {
        match &mut self.kind {
            HitBoxKind::Once(vec) => vec.clear(),
            HitBoxKind::Persistent => {}
        }
    }

    pub fn contains(&mut self, entity: Entity) -> bool {
        match &self.kind {
            HitBoxKind::Once(vec) => vec.contains(&entity),
            HitBoxKind::Persistent => false,
        }
    }
}

#[derive(Debug, Default, Component)]
pub enum HitBoxKind {
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
    mut hit_boxes: Query<(&ColliderParent, &CollidingEntities, &mut HitBox)>,
    hurt_boxes: Query<&ColliderParent, With<HurtBox>>,
    mut applyhealthdelta_evw: EventWriter<ApplyHealthDelta>,
) {
    for (parent, colliding_entities, mut hitbox) in &mut hit_boxes {
        let mut colliding_entities: Vec<Entity> = colliding_entities
            .iter()
            .filter_map(|entity| {
                if let Ok(collider_parent) = hurt_boxes.get(*entity) {
                    let parent = collider_parent.get();

                    if hurt_boxes.contains(*entity) && !hitbox.contains(parent) {
                        Some(parent)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        if !colliding_entities.is_empty() {
            println!(
                "{:?} is colliding with the following entities: {:?}",
                parent.get(),
                colliding_entities
            );

            applyhealthdelta_evw.send_batch(colliding_entities.iter().map(|entity| {
                ApplyHealthDelta {
                    entity: *entity,
                    delta: -hitbox.damage,
                }
            }));

            match hitbox.kind {
                HitBoxKind::Once(ref mut vec) => vec.append(&mut colliding_entities),
                HitBoxKind::Persistent => {}
            }
        }
    }
}

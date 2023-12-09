use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_xpbd_2d::components::{
    Collider, ColliderParent, CollidingEntities, CollisionLayers, LinearVelocity, Sensor,
};

use crate::GameState;

pub struct BehaviourPlugin;

/// This plugin handles castle related stuff like health ui
/// Castle logic is only active during the State `GameState::Playing`
impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                behaviour,
                behavior_added,
                enemy_finder,
                face_velocity_vector,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/* TODO: Add "DefaultBehaviour" Component */
#[derive(Debug, Clone, Component)]
pub enum Behaviour {
    Wandering(Timer, LinearVelocity),
    MoveToPoint(Vec2),
    MoveAndAttack(Entity),
}

impl Default for Behaviour {
    fn default() -> Self {
        Self::Wandering(
            Timer::from_seconds(2.5, TimerMode::Repeating),
            LinearVelocity::default(),
        )
    }
}

fn behaviour(
    mut query: Query<(
        Entity,
        &mut LinearVelocity,
        &mut Behaviour,
        Option<&DefaultBehaviour>,
        &CollidingEntities,
    )>,
    transforms: Query<&GlobalTransform>,
    time: Res<Time>,
    mut direction: Local<Vec2>,
) {
    for (source_entity, mut velocity, mut behaviour, default_behaviour, colliding_entities) in
        query.iter_mut()
    {
        let inner_behaviour = behaviour.as_mut();
        match inner_behaviour {
            Behaviour::Wandering(ref mut timer, ref mut saved_velocity) => {
                wandering(&time, timer, &mut velocity, saved_velocity, &mut direction)
            }
            Behaviour::MoveToPoint(dst_point) => {
                let src_point = transforms
                    .get(source_entity)
                    .unwrap()
                    .translation()
                    .truncate();
                move_to_point(&mut velocity, &src_point, dst_point, 30.0);
            }
            Behaviour::MoveAndAttack(entity) => {
                let src_point = transforms
                    .get(source_entity)
                    .unwrap()
                    .translation()
                    .truncate();
                if let Ok(dst_transform) = transforms.get(*entity) {
                    let dst_point = dst_transform.translation().truncate();
                    move_and_attack(
                        &mut velocity,
                        &src_point,
                        &dst_point,
                        colliding_entities,
                        entity,
                    );
                } else {
                    velocity.0 = Vec2::ZERO;
                    *behaviour = default_behaviour.map_or(Behaviour::default(), |b| b.0.clone());
                }
            }
        }
    }
}

fn wandering(
    time: &Time,
    timer: &mut Timer,
    velocity: &mut LinearVelocity,
    saved_velocity: &mut LinearVelocity,
    direction: &mut Vec2,
) {
    if timer.tick(time.delta()).just_finished() {
        *direction = Vec2::new((direction.x + 0.5) % 2.0, (direction.y + 0.5) % 2.0);
        let vector = *direction - Vec2::new(1.0, 1.0);
        velocity.0 = vector * 20.0;
        saved_velocity.0 = vector * 20.0;
    }

    *velocity = *saved_velocity;
}

fn move_to_point(
    velocity: &mut LinearVelocity,
    src_point: &Vec2,
    dst_point: &Vec2,
    velocity_scale: f32,
) {
    let vector = (*dst_point - *src_point).normalize() * velocity_scale;

    velocity.0 = vector;
}

fn move_and_attack(
    velocity: &mut LinearVelocity,
    src_point: &Vec2,
    dst_point: &Vec2,
    colliding_entities: &CollidingEntities,
    dst_entity: &Entity,
) {
    if !colliding_entities.contains(dst_entity) {
        let velocity_scale =
            (src_point.distance(*dst_point).min(100.0).max(1.0) - 1.0) / 100.0 * 30.0;
        move_to_point(velocity, src_point, dst_point, velocity_scale);
    }
}

#[derive(Debug, Default, Component)]
pub struct EnemyFinder;

#[derive(Debug, Default, Bundle)]
pub struct EnemyFinderBundle {
    pub enemy_finder: EnemyFinder,
    pub collider: Collider,
    pub sensor: Sensor,
    pub collisionlayers: CollisionLayers,
}

fn enemy_finder(
    mut behaviours: Query<&mut Behaviour>,
    enemy_finders: Query<(&ColliderParent, &CollidingEntities), With<EnemyFinder>>,
    transforms: Query<&GlobalTransform>,
    collider_parents: Query<&ColliderParent>,
) {
    for (parent, colliding_entities) in &enemy_finders {
        let parent = parent.get();
        /* ColliderParent has the necessary components: Behaviour and GlobalTransform */
        if let (Ok(mut behaviour), Ok(src_transform)) =
            (behaviours.get_mut(parent), transforms.get(parent))
        {
            let src_point = src_transform.translation().truncate();
            /* Behaviour is Wandering and our EnemyFinder actually collides with somethings */
            if !colliding_entities.is_empty() {
                /* Get the shortest distance Entity */
                if let Some((target, _)) = colliding_entities
                    .iter()
                    .filter_map(|entity| {
                        if let Ok(parent) = collider_parents.get(*entity) {
                            let parent = parent.get();

                            if let Ok(dst_transform) = transforms.get(parent) {
                                let dst_point = dst_transform.translation().truncate();

                                Some((parent, src_point.distance(dst_point)))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                {
                    *behaviour = Behaviour::MoveAndAttack(target);
                }
            }
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct DefaultBehaviour(pub Behaviour);

fn behavior_added(
    mut behaviours: Query<&mut Behaviour, Added<Behaviour>>,
    mut direction: Local<Vec2>,
) {
    for mut behaviour in &mut behaviours {
        if let Behaviour::Wandering(_, ref mut velocity) = behaviour.as_mut() {
            *direction = Vec2::new((direction.x + 0.5) % 2.0, (direction.y + 0.5) % 2.0);
            let vector = *direction - Vec2::new(1.0, 1.0);
            velocity.0 = vector * 20.0;
        }
    }
}

fn face_velocity_vector(mut query: Query<(&mut Transform, &LinearVelocity)>) {
    for (mut transform, velocity) in &mut query {
        let angle = velocity.0.y.atan2(velocity.0.x) + FRAC_PI_2; // Add/sub FRAC_PI here optionally
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }
}

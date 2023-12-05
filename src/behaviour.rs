use bevy::prelude::*;
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use bevy_xpbd_2d::components::{CollidingEntities, LinearVelocity};
use rand_core::RngCore;

use crate::GameState;

pub struct BehaviourPlugin;

/// This plugin handles castle related stuff like health ui
/// Castle logic is only active during the State `GameState::Playing`
impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (behaviour).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Component)]
pub enum Behaviour {
    Wandering(Timer),
    MoveToPoint(Vec2),
    MoveAndAttack(Entity),
}

impl Default for Behaviour {
    fn default() -> Self {
        Self::Wandering(Timer::from_seconds(2.5, TimerMode::Repeating))
    }
}

fn behaviour(
    mut query: Query<(
        Entity,
        &mut LinearVelocity,
        &mut Behaviour,
        &CollidingEntities,
    )>,
    transforms: Query<&GlobalTransform>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    for (source_entity, mut velocity, mut behaviour, colliding_entities) in query.iter_mut() {
        let behaviour = behaviour.as_mut();
        match behaviour {
            Behaviour::Wandering(ref mut timer) => wandering(&time, timer, &mut velocity, &mut rng),
            Behaviour::MoveToPoint(dst_point) => {
                let src_point = transforms
                    .get(source_entity)
                    .unwrap()
                    .translation()
                    .truncate();
                move_to_point(&mut velocity, &src_point, dst_point);
            }
            Behaviour::MoveAndAttack(entity) => {
                let src_point = transforms
                    .get(source_entity)
                    .unwrap()
                    .translation()
                    .truncate();
                let dst_point = transforms.get(*entity).unwrap().translation().truncate();

                move_and_attack(
                    &mut velocity,
                    &src_point,
                    &dst_point,
                    colliding_entities,
                    entity,
                );
            }
        }
    }
}

fn wandering(
    time: &Time,
    timer: &mut Timer,
    velocity: &mut LinearVelocity,
    rng: &mut GlobalEntropy<ChaCha8Rng>,
) {
    if timer.tick(time.delta()).just_finished() {
        let vector = Vec2::new(
            ((rng.next_u32() % 200) as f32 - 100.0) / 100.0,
            ((rng.next_u32() % 200) as f32 - 100.0) / 100.0,
        );
        velocity.0 = vector * 20.0;
    }
}

fn move_to_point(velocity: &mut LinearVelocity, src_point: &Vec2, dst_point: &Vec2) {
    let vector = (*dst_point - *src_point).normalize() * 30.0;

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
        move_to_point(velocity, src_point, dst_point);
    }
}

use bevy::prelude::*;
use std::{fmt::Display, ops::Add};

use crate::{castle::UnitPoints, units::Faction, GameState};

pub struct AttributesPlugin;

/// This plugin handles attributes related stuff like health
/// Attribure logic is only active during the State `GameState::Playing`
impl Plugin for AttributesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyHealthDelta>().add_systems(
            Update,
            (apply_health_delta, die).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn apply(&mut self, delta: f32) {
        self.current = self.current.add(delta).max(0.0).min(self.max);
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0:.2}/{1:.2}", self.current, self.max))
    }
}

#[derive(Debug, Event)]
pub struct ApplyHealthDelta {
    pub entity: Entity,
    pub delta: f32,
}

fn apply_health_delta(
    mut applyhealthdelta_evr: EventReader<ApplyHealthDelta>,
    mut health_query: Query<&mut Health>,
) {
    for ev in applyhealthdelta_evr.read() {
        println!("{:?}", ev);
        if let Ok(mut health) = health_query.get_mut(ev.entity) {
            health.apply(ev.delta);
        }
    }
}

fn die(
    mut commands: Commands,
    health_query: Query<(Entity, &Health, &Faction), (Changed<Health>, Without<Immortal>)>,
    mut unit_points: ResMut<UnitPoints>,
) {
    for (entity, health, faction) in &health_query {
        if health.current == 0.0 {
            commands.entity(entity).despawn_recursive();

            if let Faction::Enemy = faction {
                unit_points.0 += 2;
            }
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct Immortal;

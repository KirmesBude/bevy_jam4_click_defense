pub mod attributes;

use bevy::prelude::*;

use crate::physics::SensorLayers;

use self::attributes::AttributesPlugin;

pub struct CommonPlugin;

/// This plugin handles attributes related stuff like health
/// Attribure logic is only active during the State `GameState::Playing`
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttributesPlugin);
    }
}

#[derive(Debug, Component, Clone, Copy)]
pub enum Faction {
    Ally,
    Enemy,
}

impl Faction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Ally => Self::Enemy,
            Self::Enemy => Self::Ally,
        }
    }

    pub fn hurt_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHurt,
            Faction::Enemy => SensorLayers::EnemyHurt,
        }
    }

    pub fn hit_layer(&self) -> SensorLayers {
        match self {
            Faction::Ally => SensorLayers::AllyHit,
            Faction::Enemy => SensorLayers::EnemyHit,
        }
    }
}

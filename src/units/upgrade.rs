use bevy::prelude::*;

pub struct UpgradePlugin;

/// This plugin handles attributes related stuff like health
/// Attribure logic is only active during the State `GameState::Playing`
impl Plugin for UpgradePlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Default, Component)]
pub struct ShieldUpgrade {
    level: usize,
}

impl ShieldUpgrade {
    const MAX_LEVEL: usize = 10;

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn level_up(&mut self) -> bool {
        if self.level < Self::MAX_LEVEL {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn cost(&self) -> usize {
        (self.level + 1) * 3
    }

    pub fn get(&self) -> f32 {
        (self.level as f32) * 12.5
    }
}

#[derive(Debug, Default, Component)]
pub struct AttackCooldownUpgrade {
    level: usize,
}

impl AttackCooldownUpgrade {
    const BASE_VALUE: f32 = 0.075;
    const MAX_LEVEL: usize = 10;

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn level_up(&mut self) -> bool {
        if self.level < Self::MAX_LEVEL {
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn cost(&self) -> usize {
        (self.level + 1) * 3
    }

    pub fn get(&self) -> f32 {
        (self.level as f32) * Self::BASE_VALUE
    }
}

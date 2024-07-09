use super::combat::{Defense, Health, Power, SufferDamage};
use bevy::prelude::Bundle;

#[derive(Debug, PartialEq, Eq, Clone, Bundle)]
pub struct CombatStats {
    health: Health,
    power: Power,
    defense: Defense,
    damage: SufferDamage,
}

impl CombatStats {
    pub fn new(health: i32, power: i32, defense: i32) -> Self {
        Self {
            health: Health::new(health),
            power: Power(power),
            defense: Defense(defense),
            damage: SufferDamage::new(),
        }
    }
}

use bevy::prelude::Component;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Health {
    pub max: i32,
    pub current: i32,
    pub min: i32,
}

impl Health {
    pub fn new(current: i32) -> Self {
        Self {
            max: current,
            current,
            min: 0,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.current -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.current <= self.min
    }
}

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Power(pub i32);

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Defense(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new() -> Self {
        Self { amount: vec![] }
    }

    pub fn add_damage(&mut self, damage: i32) {
        self.amount.push(damage)
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, i32> {
        self.amount.drain(..)
    }
}

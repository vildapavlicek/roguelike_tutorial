use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
pub struct Item;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
pub struct Potion {
    pub amount: i32,
}

impl Potion {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

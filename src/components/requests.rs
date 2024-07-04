//! Request components are components that request to do some kind of action. For example if entity wants to move, then we mark the entity with the component [MovementRequest]. Each request component should carry clear intention of what action is requested to be done.
//!
//!
//!
use bevy::prelude::{Component, Entity, Visibility};

/// Component to request movement. The values indicate bych how much the entity should move by.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Component)]
pub struct MovementRequest {
    pub x: i32,
    pub y: i32,
}

impl MovementRequest {
    pub fn up() -> Self {
        MovementRequest { x: 0, y: 1 }
    }
    pub fn right() -> Self {
        MovementRequest { x: 1, y: 0 }
    }
    pub fn down() -> Self {
        MovementRequest { x: 0, y: -1 }
    }
    pub fn left() -> Self {
        MovementRequest { x: -1, y: 0 }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Component)]
pub struct VisibilityChangeRequest {
    pub entity: Entity,
    pub new_visibility: Visibility,
}

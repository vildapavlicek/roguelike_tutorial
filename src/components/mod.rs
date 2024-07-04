use crate::consts::{PLAYER_Z, SPRITE_SIZE};
use bevy::{
    prelude::{Component, Vec3},
    utils::hashbrown::HashSet,
};
use std::{
    hash::Hash,
    ops::{Add, AddAssign},
};

pub mod requests;

/// Our position custom position component used for tracking entity's position in a grid using int.
/// This position is to simply some logic and to keep more consistent with the tutorial. It gets translates into [bevy::prelude::Transform] by system.
/// Implements  [Add], [AddAssign] and [Hash] manually. When using implementing those traits, only `x` and `y` fields are taken into account. For our 2D map only those two are important for position related information. `z` is only used for ordering what will be rendered on what.
#[derive(Debug, Eq, Component, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn test() {
    assert_eq!(
        Position { x: 1, y: 1, z: 55 },
        Position { x: 1, y: 1, z: 33 }
    )
}

impl Add<requests::MovementRequest> for Position {
    type Output = Position;

    fn add(self, rhs: requests::MovementRequest) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
        }
    }
}

impl AddAssign<requests::MovementRequest> for Position {
    fn add_assign(&mut self, rhs: requests::MovementRequest) {
        *self = *self + rhs
    }
}

impl AddAssign<&requests::MovementRequest> for Position {
    fn add_assign(&mut self, rhs: &requests::MovementRequest) {
        *self = *self + *rhs
    }
}

impl From<Position> for Vec3 {
    fn from(Position { x, y, z }: Position) -> Self {
        Vec3::new(x as f32 * SPRITE_SIZE, y as f32 * SPRITE_SIZE, z as f32)
    }
}

impl From<&Position> for Vec3 {
    fn from(Position { x, y, z }: &Position) -> Self {
        Vec3::new(*x as f32 * SPRITE_SIZE, *y as f32 * SPRITE_SIZE, *z as f32)
    }
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Player;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Wall;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Floor;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Impassable;

#[derive(Debug, PartialEq, Eq, Component, Clone)]
pub struct Viewshed {
    pub visible_range: u8,
    pub visible_tiles: HashSet<Position>,
}

impl Viewshed {
    pub fn new(visible_range: u8) -> Self {
        Self {
            visible_range,
            visible_tiles: HashSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Visible;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Revealed;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Monster;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct FogOfWar;

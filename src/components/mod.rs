use crate::consts::SPRITE_SIZE;
use bevy::{
    prelude::{Component, Vec3},
    utils::hashbrown::HashSet,
};
use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign},
};

pub mod requests;

#[derive(Debug, Eq, PartialEq, Component, Clone)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Name {
    fn default() -> Self {
        Name(String::from("Name not set"))
    }
}

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

    /// This is helper for A* algo and returns all possible directions we can move in
    pub fn possible_successors(&self) -> Vec<Position> {
        vec![
            // up
            Position::new(self.x, self.y + 1, self.z),
            // up, right
            Position::new(self.x + 1, self.y + 1, self.z),
            // right
            Position::new(self.x + 1, self.y, self.z),
            // right, down
            Position::new(self.x + 1, self.y - 1, self.z),
            // down
            Position::new(self.x, self.y - 1, self.z),
            // down, left
            Position::new(self.x - 1, self.y - 1, self.z),
            // left
            Position::new(self.x - 1, self.y, self.z),
            // left, up
            Position::new(self.x - 1, self.y + 1, self.z),
        ]
    }

    pub fn distance(self, rhs: Position) -> i32 {
        let dx = (self.x - rhs.x) as f64;
        let dy = (self.y - rhs.y) as f64;
        ((dx * dx + dy * dy).sqrt()) as i32
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

/// Marks entity that will be controlled by player
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Player;

/// Marks entity as a wall, it blocks player from going through as well as blocks sight
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Wall;

/// entity which is a floor that player can step on
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Floor;

/// Not sure about this one, it should mark entities that cannot be walked through
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Impassable;

/// Defines players field of view
#[derive(Debug, PartialEq, Eq, Component, Clone)]
pub struct Viewshed {
    /// how far player can see
    pub visible_range: u8,
    /// which positions are currently in player's field of view
    pub visible_tiles: HashSet<Position>,
}

impl Viewshed {
    pub fn new(visible_range: u8) -> Self {
        Self {
            visible_range,
            visible_tiles: HashSet::new(),
        }
    }

    pub fn visible_range(&self) -> u8 {
        self.visible_range
    }

    pub fn set_visible_tiles(&mut self, visible_tiles: HashSet<Position>) {
        self.visible_tiles = visible_tiles
    }

    pub fn contains(&self, pos: &Position) -> bool {
        self.visible_tiles.contains(pos)
    }
}

/// Marks entities which are currently inside player's field of view
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Visible;

/// Entitties that entered into player's field of view at least once
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Revealed;

/// Tag to mark entities that are monsters hostile to player
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct Monster;

/// Used to mark entities that will be hidden by Fog of War when not in player's field of view
#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct FogOfWar;

#[derive(Debug, PartialEq, Eq, Component, Clone, Copy)]
pub struct BlocksSight;

use crate::components::Position;
use bevy::{math::Vec2, prelude::Resource};

#[derive(Debug, Clone, Resource)]
pub struct SpawnPoints {
    pub player: Position,
    pub monsters: Vec<Position>,
}

#[derive(Debug, Copy, Clone, Resource)]
pub struct CursorPosition {
    pub world_position: Position,
    pub window_position: Vec2,
}

impl CursorPosition {
    pub fn set(&mut self, world_position: Position, window_position: Vec2) {
        self.world_position = world_position;
        self.window_position = window_position;
    }

    pub fn world_position(&self) -> &Position {
        &self.world_position
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        // when spawning map, every position is greater than 0, due to map being generated through vector and positions are based on the index
        // so having starting position of cursor to be -99 should be safe
        CursorPosition {
            world_position: Position {
                x: -99,
                y: -99,
                z: -99,
            },
            window_position: Vec2::splat(-99f32),
        }
    }
}

use crate::components::Position;
use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct SpawnPoints {
    pub player: Position,
    pub monsters: Vec<Position>,
}

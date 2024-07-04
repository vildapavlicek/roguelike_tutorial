use crate::{components::Position, consts::PLAYER_Z};
use bevy::prelude::Resource;

#[derive(Debug, Clone, Resource)]
pub struct SpawnPoints {
    pub player: Position,
    pub monsters: Vec<Position>,
}

impl SpawnPoints {
    pub fn new(player: Position, monsters: Vec<Position>) -> Self {
        SpawnPoints { player, monsters }
    }
}

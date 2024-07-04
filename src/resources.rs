use bevy::prelude::Resource;

use crate::consts::PLAYER_Z;

#[derive(Debug, Clone, Copy, Resource)]
pub struct PlayerSpawnPoint(crate::components::Position);

impl PlayerSpawnPoint {
    pub fn new(x: i32, y: i32) -> Self {
        PlayerSpawnPoint(crate::components::Position {
            x,
            y,
            z: PLAYER_Z as i32,
        })
    }

    pub fn inner(&self) -> crate::components::Position {
        self.0
    }
}

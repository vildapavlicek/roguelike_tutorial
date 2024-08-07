use bevy::prelude::States;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    Menu,
    #[default]
    PlayerTurn,
    EnemyTurn,
    PlayerDead,
}

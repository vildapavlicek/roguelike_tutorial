use crate::{
    components::{requests::MovementRequest, Position},
    states::GameState,
};
use bevy::{app::Startup, prelude::*};

mod map;
mod monster;
mod player;

pub struct InitSetup;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct InitSetupSet;

impl Plugin for InitSetup {
    fn name(&self) -> &str {
        "Default systems set up"
    }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(GameState::default())
            .add_systems(
                Startup,
                (spawn_camera, map::spawn)
                    .in_set(InitSetupSet)
                    .run_if(run_once()),
            )
            .add_plugins((player::PlayerPlugin, monster::MonsterPlugin));
    }
}

/// Spawns 2D camera, so we can see stuff
fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

/// Processes movement, takes each [MovementRequest] and updates position accordingly
fn process_movement(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Position, &MovementRequest)>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut position, movement_request)| {
            *position += movement_request;
            cmd.entity(entity).remove::<MovementRequest>();
        });
}

/// Internally we track position with [Position] but bevy uses [Transform]. This system then synces these two, by updating [Transform] base on the [Position]
fn sync_position(
    position: Query<(Entity, &Position), Changed<Position>>,
    mut transforms: Query<&mut Transform>,
) {
    position.iter().for_each(|(ent, pos)| {
        if let Ok(mut transform) = transforms.get_mut(ent) {
            transform.translation = pos.into();
        };
    });
}

use crate::{
    components::{
        requests::MovementRequest, Floor, FogOfWar, Monster, Player, Position, Revealed, Visible,
        Wall,
    },
    consts::FOW_ALPHA,
};
use bevy::{
    app::{Startup, Update},
    log::trace,
    prelude::{
        run_once, Camera2d, Camera2dBundle, Changed, Commands, Entity, IntoSystemConfigs, Mut, Or,
        Plugin, Query, Sprite, SystemSet, Transform, With, Without,
    },
};

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
        app.add_systems(
            Startup,
            (spawn_camera, map::spawn)
                .in_set(InitSetupSet)
                .run_if(run_once()),
        )
        .add_plugins((player::PlayerPlugin, monster::MonsterPlugin));
    }
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

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

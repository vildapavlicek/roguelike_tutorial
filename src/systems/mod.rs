use crate::{
    components::{requests::MovementRequest, InFov, Player, Position, Revealed},
    consts::FOW_ALPHA,
};
use bevy::{
    app::{Startup, Update},
    log::trace,
    prelude::{
        run_once, Camera2d, Camera2dBundle, Changed, Commands, Entity, IntoSystemConfigs, Mut,
        Plugin, Query, Sprite, SystemSet, Transform, With, Without,
    },
};

mod map;
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
            (spawn_camera, map::spawn_map)
                .in_set(InitSetupSet)
                .run_if(run_once()),
        )
        .add_systems(Update, apply_fow)
        .add_plugins(player::PlayerPlugin);
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

fn sync_camera_with_player(
    player_pos: Query<&Transform, With<Player>>,
    mut camera_pos: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera_pos.single_mut().translation = player_pos.single().translation
}

fn apply_fow(mut query: Query<&mut Sprite, (With<Revealed>, Without<InFov>)>) {
    fn apply_alpha(mut sprite: Mut<Sprite>) {
        if sprite.color.a() >= 1f32 {
            sprite.color.set_a(FOW_ALPHA);
        }
    }
    query.iter_mut().for_each(apply_alpha)
}

use crate::{
    components::{Monster, Viewshed},
    resources::SpawnPoints,
};
use bevy::{
    app::Startup,
    log::trace,
    prelude::{
        default, run_once, AssetServer, Commands, IntoSystemConfigs, Plugin, Res, SpriteBundle,
    },
};

pub(super) struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            spawn_monsters.run_if(run_once()).after(super::InitSetupSet),
        );
    }
}

pub(super) fn spawn_monsters(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    spawn_points: Res<SpawnPoints>,
) {
    trace!("spawning monsters");
    let goblin = asset_server.load("goblin.png");

    spawn_points.monsters.iter().for_each(|spawn_point| {
        cmd.spawn(SpriteBundle {
            texture: goblin.clone(),
            ..default()
        })
        .insert(*spawn_point)
        .insert(Viewshed::new(4))
        .insert(Monster);
    })
}

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
use rand::Rng;

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
    let orc = asset_server.load("orc.png");

    spawn_points.monsters.iter().for_each(|spawn_point| {
        let texture = (rand::thread_rng().gen_range(0f32..1f32) > 0.75f32)
            .then(|| orc.clone())
            .unwrap_or(goblin.clone());
        cmd.spawn(SpriteBundle {
            visibility: bevy::render::view::Visibility::Hidden,
            texture,
            ..default()
        })
        .insert(*spawn_point)
        .insert(Viewshed::new(4))
        .insert(Monster);
    })
}

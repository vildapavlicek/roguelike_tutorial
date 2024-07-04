use bevy::prelude::*;

mod algorithms;
mod components;
mod consts;
mod resources;
mod systems;
mod utils;

fn main() {
    println!("Hello, world!");

    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "roguelike_tutorial".to_string(),
                        resolution: (1024f32, 768f32).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    filter: "roguelike_tutorial=trace".to_string(),
                    ..default()
                }),
            systems::InitSetup,
        ))
        .run();
}

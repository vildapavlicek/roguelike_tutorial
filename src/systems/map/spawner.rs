use super::rect::Rect;
use crate::{
    ai::*,
    components::{bundles::*, *},
    consts::{FLOOR_Z, ITEM_Z, MONSTER_Z, SPRITE_SIZE},
};
use bevy::{
    prelude::{
        default, AssetServer, Commands, Handle, Image, Res, SpriteBundle, Transform, Vec3,
        Visibility,
    },
    utils::hashbrown::HashSet,
};
use big_brain::{pickers::FirstToScore, thinker::Thinker};
use item::{Item, Potion};
use rand::Rng;

pub(super) fn spawn_monster(
    cmd: &mut Commands,
    position: Position,
    asset_server: &Res<AssetServer>,
) {
    match (rand::thread_rng().gen_range(0f32..1f32) > 0.75f32) {
        true => spawn_orc(cmd, position, asset_server.load("orc.png")),
        false => spawn_goblin(cmd, position, asset_server.load("goblin.png")),
    }
}

pub(super) fn spawn_orc(cmd: &mut Commands, position: Position, texture: Handle<Image>) {
    cmd.spawn((
        SpriteBundle {
            visibility: bevy::render::view::Visibility::Hidden,
            texture,
            ..default()
        },
        position,
        Viewshed::new(4),
        Monster,
        BlocksSight,
        Name("Orc".into()),
        CombatStats::new(16, 4, 1),
        Thinker::build()
            .picker(FirstToScore { threshold: 0.5 })
            .when(PlayerInAttackRange, MeeleeAttackPlayer)
            .when(PlayerVisible, ChasePlayer),
    ));
}

pub(super) fn spawn_goblin(cmd: &mut Commands, position: Position, texture: Handle<Image>) {
    cmd.spawn((
        SpriteBundle {
            visibility: Visibility::Hidden,
            texture,
            ..default()
        },
        position,
        Viewshed::new(4),
        Monster,
        BlocksSight,
        Name("Goblin".into()),
        CombatStats::new(16, 4, 1),
        Thinker::build()
            .picker(FirstToScore { threshold: 0.5 })
            .when(PlayerInAttackRange, MeeleeAttackPlayer)
            .when(PlayerVisible, ChasePlayer),
    ));
}

pub(super) fn spawn_wall(cmd: &mut Commands, position: Position, texture: Handle<Image>) {
    cmd.spawn((
        SpriteBundle {
            texture: texture,
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::new(
                position.x as f32 * SPRITE_SIZE,
                position.y as f32 * SPRITE_SIZE,
                FLOOR_Z,
            )),
            ..default()
        },
        BlocksSight,
        Wall,
        BlocksTile,
        FogOfWar,
        position,
    ));
}

pub(super) fn spawn_floor(cmd: &mut Commands, position: Position, texture: Handle<Image>) {
    cmd.spawn((
        SpriteBundle {
            texture: texture.clone(),
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::new(
                position.x as f32 * SPRITE_SIZE,
                position.y as f32 * SPRITE_SIZE,
                FLOOR_Z,
            )),
            ..default()
        },
        position,
        Floor,
        FogOfWar,
    ));
}

pub(super) fn spawn_health_potions(
    cmd: &mut Commands,
    positions: Vec<Position>,
    asset_server: &Res<AssetServer>,
) {
    positions
        .into_iter()
        .for_each(|position| spawn_potion(cmd, position, asset_server));
}

pub(super) fn spawn_potion(
    cmd: &mut Commands,
    position: Position,
    asset_server: &Res<AssetServer>,
) {
    let texture = asset_server.load("health_potion.png");
    cmd.spawn((
        SpriteBundle {
            texture,
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::new(
                position.x as f32 * SPRITE_SIZE,
                position.y as f32 * SPRITE_SIZE,
                FLOOR_Z,
            )),
            ..default()
        },
        position,
        Item,
        Potion::new(8),
        Name::new("Health Potion"),
    ));
}

pub(super) fn populate_room(
    cmd: &mut Commands,
    room: &Rect,
    max_monsters: u8,
    max_items: u8,
    asset_server: &Res<AssetServer>,
) {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    enum Spawn {
        Monster(Position),
        Item(Position),
    }

    let mut rng = rand::thread_rng();
    let monsters_count = rng.gen_range(0..=max_monsters);
    let mut spawn_points = HashSet::new();

    for _ in 0..monsters_count {
        let mut added = false;

        while !added {
            let (x, y) = room.rand_position();
            let spawn_point = Spawn::Monster(Position::new(x as i32, y as i32, MONSTER_Z as i32));
            if spawn_points.contains(&spawn_point) {
                continue;
            }

            spawn_points.insert(spawn_point);
            added = true;
        }
    }

    let items_count = rng.gen_range(0..=max_items);

    for _ in 0..items_count {
        let mut added = false;

        while !added {
            let (x, y) = room.rand_position();
            let spawn_point = Spawn::Item(Position::new(x as i32, y as i32, ITEM_Z as i32));
            if spawn_points.contains(&spawn_point) {
                continue;
            }

            spawn_points.insert(spawn_point);
            added = true;
        }
    }

    spawn_points
        .into_iter()
        .for_each(|to_spawn| match to_spawn {
            Spawn::Item(position) => spawn_potion(cmd, position, asset_server),
            Spawn::Monster(position) => spawn_monster(cmd, position, asset_server),
        });
}

pub(super) fn spawn_player(
    cmd: &mut Commands,
    position: Position,
    asset_server: &Res<AssetServer>,
) {
    let texture = asset_server.load("hooded.png");

    cmd.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        position,
        crate::components::Player,
        Viewshed::new(10),
        Name::new("Player"),
        CombatStats::new(30, 5, 2),
    ));
}

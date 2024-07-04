mod rect;

use crate::{
    components::{Floor, Impassable, Position, Wall},
    consts::{FLOOR_Z, SPRITE_SIZE, WALL_Z},
};
use bevy::{
    asset::AssetServer,
    log::{debug, trace},
    prelude::{
        default, Color, Commands, Res, Resource, Sprite, SpriteBundle, Transform, Vec2, Vec3,
    },
    render::view::Visibility,
};
use rand::Rng;
use rect::Rect;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Clone, Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    // todo could be renamed to `new_dungeon` but keep the current name to have it same as the tutorial
    fn new_map_rooms_and_corridors() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: vec![],
            width: 80,
            height: 50,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(MIN_SIZE..MAX_SIZE);
            let h = rand::thread_rng().gen_range(MIN_SIZE..MAX_SIZE);
            let x = rand::thread_rng().gen_range(1..80 - w - 1) - 1;
            let y = rand::thread_rng().gen_range(1..50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            if !map.rooms.iter().any(|other| new_room.intersect(other)) {
                map.apply_room_to_map(&new_room);

                let (new_x, new_y) = new_room.center();
                let Some((prev_x, prev_y)) = map.rooms.iter().last().map(Rect::center) else {
                    map.rooms.push(new_room);
                    continue;
                };
                if rand::thread_rng().gen_range(0..2) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

pub(super) fn spawn_map(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let floor = asset_server.load("cave_floor_dark.png");
    let wall = asset_server.load("wall.png");

    let mut x = 0;
    let mut y = 0;
    trace!("generating new map");
    let map = Map::new_map_rooms_and_corridors();

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                cmd.spawn(SpriteBundle {
                    texture: floor.clone(),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * SPRITE_SIZE,
                        y as f32 * SPRITE_SIZE,
                        FLOOR_Z,
                    )),
                    ..default()
                })
                .insert(Position::new(x, y, FLOOR_Z as i32))
                .insert(Floor);
            }
            TileType::Wall => {
                cmd.spawn(SpriteBundle {
                    texture: wall.clone(),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * SPRITE_SIZE,
                        y as f32 * SPRITE_SIZE,
                        FLOOR_Z,
                    )),
                    ..default()
                })
                .insert(Position::new(x, y, WALL_Z as i32))
                .insert(Wall)
                .insert(Impassable);
            }
        };

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        };
    }

    let room_index = rand::thread_rng().gen_range(0..map.rooms.len());
    let (x, y) = map
        .rooms
        .get(room_index)
        .map(Rect::center)
        .expect("No rooms to spawn player in");
    cmd.insert_resource(crate::resources::PlayerSpawnPoint::new(x, y));
}

// this was first iteration of the map generator and is unused
// it doesn't work after refactoring and tbh I'm too lazy to fix it
// fn new_map() -> Vec<TileType> {
//     let x_max = 80;
//     let y_max = 50;
//     let map_size = x_max * y_max;

//     let mut map = vec![TileType::Floor; map_size];

//     // boundary walls
//     for x in 0..80 {
//         map[xy_idx(x, 0)] = TileType::Wall;
//         map[xy_idx(x, 49)] = TileType::Wall;
//     }

//     for y in 0..50 {
//         map[xy_idx(0, y)] = TileType::Wall;
//         map[xy_idx(79, y)] = TileType::Wall;
//     }

//     for _ in 0..(map_size / 2) {
//         let x = rand::thread_rng().gen_range(1..x_max) as i32;
//         let y = rand::thread_rng().gen_range(1..y_max) as i32;
//         let idx = xy_idx(x, y);
//         // 40, 25 is player starting position
//         if idx != xy_idx(40, 25) {
//             map[idx] = TileType::Wall;
//         }
//     }

//     map
// }

fn corridor_map() -> Map {
    let mut map = Map {
        height: 50,
        width: 80,
        rooms: vec![],
        tiles: vec![TileType::Wall; 80 * 50],
    };

    map.apply_vertical_tunnel(2, 20, 2);

    map
}

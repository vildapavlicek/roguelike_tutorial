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
use std::{
    cmp::{max, min},
    ops::Sub,
    usize,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Clone, Resource)]
pub struct Map {
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn xy_idx(&self, x: usize, y: usize) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: usize, x2: usize, y: usize) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: usize, y2: usize, x: usize) {
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

        const MAX_ROOMS: usize = 30;
        const MIN_SIZE: usize = 6;
        const MAX_SIZE: usize = 10;

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

    // TODO: doesn't work at the moment, it needs to be fixed
    fn next_to_floor(&self, x: usize, y: usize) -> bool {
        let index = self.xy_idx(x, y);

        let up = index + self.width;
        let right = index + 1;
        let down = index.checked_sub(self.width);
        let left = index.checked_sub(1);

        fn is_floor(tile: &TileType) -> bool {
            matches!(tile, TileType::Floor)
        }

        // if right tile is floor, return true
        if self.tiles.get(right).map(is_floor).unwrap_or_default() {
            return true;
        }

        // if the tile on the left is floor, return true
        if left
            .and_then(|index| self.tiles.get(index))
            .map(is_floor)
            .unwrap_or_default()
        {
            return true;
        }

        // if tile above is floor, return true
        if self.tiles.get(up).map(is_floor).unwrap_or_default() {
            return true;
        }

        // if tile below is floor, return true
        if down
            .and_then(|index| self.tiles.get(index))
            .map(is_floor)
            .unwrap_or_default()
        {
            return true;
        }

        let up_right = up + 1;
        let up_left = up - 1;
        let down_left = down.and_then(|v| v.checked_sub(1));
        let down_right = down.map(|v| v + 1);

        // if tile above and to the right is floor, return true
        if self.tiles.get(up_right).map(is_floor).unwrap_or_default() {
            return true;
        }

        // if tile above and to the left is floor, return true
        if self.tiles.get(up_left).map(is_floor).unwrap_or_default() {
            return true;
        }

        // if tile below and to the right is floor, return true
        if down_right
            .and_then(|index| self.tiles.get(index))
            .map(is_floor)
            .unwrap_or_default()
        {
            return true;
        }

        // if tile below and to the left is floor, return true
        if down_left
            .and_then(|index| self.tiles.get(index))
            .map(is_floor)
            .unwrap_or_default()
        {
            return true;
        }

        false
    }
}

pub(super) fn spawn_map(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let floor = asset_server.load("cave_floor_dark.png");
    let wall = asset_server.load("wall.png");

    // let mut x = 0;
    // let mut y = 0;
    trace!("generating new map");
    let map = Map::new_map_rooms_and_corridors();

    for (index, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(index);
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
                .insert(Position::new(x as i32, y as i32, FLOOR_Z as i32))
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
                .insert(Position::new(x as i32, y as i32, WALL_Z as i32))
                .insert(Wall)
                .insert(Impassable);
            }
        };

        // x += 1;
        // if x > 79 {
        //     x = 0;
        //     y += 1;
        // };
    }

    let room_index = rand::thread_rng().gen_range(0..map.rooms.len());
    let (x, y) = map
        .rooms
        .get(room_index)
        .map(Rect::center)
        .expect("No rooms to spawn player in");
    cmd.insert_resource(crate::resources::PlayerSpawnPoint::new(x as i32, y as i32));
}

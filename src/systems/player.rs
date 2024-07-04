use crate::{
    components::{
        requests::{MovementRequest, VisibilityChangeRequest},
        Floor, FogOfWar, Impassable, Monster, Player, Position, Revealed, Viewshed, Visible, Wall,
    },
    consts::{FOW_ALPHA, PLAYER_Z, SPRITE_SIZE},
};
use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    log::{debug, trace},
    prelude::{
        default, run_once, Camera2d, Changed, Color, Commands, DetectChanges, Entity,
        IntoSystemConfigs, KeyCode, Mut, Or, Plugin, Query, Ref, RemovedComponents, Res, ResMut,
        Sprite, SpriteBundle, Startup, Transform, Update, Vec2, With, Without,
    },
    render::view::{visibility, Visibility},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Player Plugin"
    }

    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            (spawn_player, compute_fov, update_visibility)
                .chain()
                .run_if(run_once())
                .after(super::InitSetupSet),
        )
        .add_systems(
            Update,
            (
                player_input,
                super::process_movement,
                super::sync_position,
                sync_camera_with_player,
                (compute_fov, update_visibility, apply_fow)
                    .chain()
                    .run_if(player_moved),
            )
                .chain(),
        );
    }
}

pub(super) fn spawn_player(
    mut cmd: Commands,
    spawn_point: Res<crate::resources::SpawnPoints>,
    asset_server: Res<AssetServer>,
) {
    let player_sprite = asset_server.load("hooded.png");

    cmd.spawn(SpriteBundle {
        texture: player_sprite,
        ..default()
    })
    .insert(spawn_point.player)
    // .insert(Position::new(0, 0, PLAYER_Z as i32))
    .insert(crate::components::Player)
    .insert(Viewshed::new(4));
}

pub(super) fn player_input(
    mut cmd: Commands,
    mut player: Query<(Entity, &Position, &mut Sprite), With<Player>>,
    input: ResMut<ButtonInput<KeyCode>>,
    impassable: Query<&Position, With<Impassable>>,
) {
    let (player_ent, player_pos, mut sprite) = player.single_mut();

    let (mut x, mut y) = (0, 0);

    // by default we look to the left, so we flip on right move
    if input.just_pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::Numpad6) {
        x += 1;
        sprite.flip_x = true;
    }

    if input.just_pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::Numpad4) {
        x -= 1;
        sprite.flip_x = false;
    }

    if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::Numpad8) {
        y += 1;
    }
    if input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::Numpad2) {
        y -= 1;
    }

    // no movement
    if x == 0 && y == 0 {
        return;
    }

    if impassable
        .iter()
        .any(|pos| *pos == *player_pos + MovementRequest { x, y })
    {
        return;
    }

    cmd.entity(player_ent).insert(MovementRequest { x, y });
}

fn player_moved(query: Query<Ref<Position>, With<Player>>) -> bool {
    query.single().is_changed()
}

fn compute_fov(
    mut player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    walls: Query<&Position, With<Wall>>,
) {
    let (p_position, mut viewshed) = player_pos.single_mut();
    viewshed.visible_tiles = crate::algorithms::my_fov::MyVisibility::new(
        |x, y| walls.iter().any(|pos| pos.x == x && pos.y == y),
        |x, y| euclidean_distance(0, 0, x, y),
    )
    .compute(*p_position, viewshed.visible_range as i32);
}

fn update_visibility(
    mut cmd: Commands,
    visible: Query<(Entity, &Position), With<Visible>>,
    mut visibility: Query<
        (Entity, &mut Visibility, &Position, Option<&Revealed>),
        Without<Visible>,
    >,
    player_visible_tiles: Query<&Viewshed, With<Player>>,
) {
    let viewshed = player_visible_tiles.single();
    visible.iter().for_each(|(entity, position)| {
        if !viewshed.visible_tiles.contains(position) {
            cmd.entity(entity).remove::<Visible>();
        }
    });

    visibility
        .iter_mut()
        .filter_map(|(entity, visibility, pos, revealed)| {
            viewshed
                .visible_tiles
                .contains(pos)
                .then(|| (entity, visibility, revealed))
        })
        .for_each(|(entity, visibility, revealed)| {
            match revealed {
                Some(_) => {
                    cmd.entity(entity).insert(Visible);
                }
                None => {
                    cmd.entity(entity).insert(Visible).insert(Revealed);
                }
            }

            set_visible(visibility);
        });
}

fn set_visible(mut visibility: Mut<Visibility>) {
    if matches!(*visibility, Visibility::Visible) {
        return;
    }

    *visibility = Visibility::Visible;
}

fn apply_fow(
    mut removed: RemovedComponents<Visible>,
    mut sprites: Query<&mut Sprite, Without<Visible>>,
    mut visited_sprites: Query<&mut Sprite, (With<Visible>, With<Revealed>)>,
) {
    fn set_fow_alpha(mut sprite: Mut<Sprite>) {
        if sprite.color.a() >= 1f32 {
            sprite.color.set_a(FOW_ALPHA);
        }
    }

    fn set_opaque(mut sprite: Mut<Sprite>) {
        if sprite.color.a() < 1f32 {
            sprite.color.set_a(1f32);
        }
    }

    // sprites that have removed component [Visible] should be hidden, ie set alpha to 25%
    removed.read().for_each(|entity| {
        sprites.get_mut(entity).map(set_fow_alpha).ok();
    });

    // sprites that are hidden, but now are in FoV
    visited_sprites.iter_mut().for_each(set_opaque)
}

fn euclidean_distance(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> i32 {
    let dx = (p1_x - p2_x) as f64;
    let dy = (p1_y - p2_y) as f64;
    ((dx * dx + dy * dy).sqrt()) as i32
}

fn sync_camera_with_player(
    player_pos: Query<&Transform, With<Player>>,
    mut camera_pos: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera_pos.single_mut().translation = player_pos.single().translation
}

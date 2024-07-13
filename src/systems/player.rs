use crate::components::requests::MeeleeAttackRequest;
use crate::components::BlocksSight;
use crate::states::GameState;
use crate::{
    components::{
        self, requests::MovementRequest, BlocksTile, FogOfWar, Monster, Name, Player, Position,
        Revealed, Viewshed, Visible,
    },
    consts::FOW_ALPHA,
};
use bevy::{asset::AssetServer, input::ButtonInput, prelude::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct PlayerTurnSet;

/// This plugin encapsulates all the systems that manage player's behiavour
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
                player_input.run_if(in_state(GameState::PlayerTurn)),
                super::process_movement,
                super::sync_position,
                sync_camera_with_player,
                (compute_fov, update_visibility, apply_fow).chain(),
            )
                .chain(),
        );
    }
}

/// This system is responsible for spawning player
fn spawn_player(
    mut cmd: Commands,
    spawn_point: Res<crate::resources::SpawnPoints>,
    asset_server: Res<AssetServer>,
) {
    let player_sprite = asset_server.load("hooded.png");

    cmd.spawn((
        SpriteBundle {
            texture: player_sprite,
            ..default()
        },
        spawn_point.player,
        crate::components::Player,
        Viewshed::new(4),
        Name::new("Player"),
        components::bundles::CombatStats::new(30, 5, 2),
    ));
}

/// This system handles user's input controlling player
pub fn player_input(
    mut cmd: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut player: Query<(Entity, &Position, &mut Sprite), With<Player>>,
    input: ResMut<ButtonInput<KeyCode>>,
    impassable: Query<&Position, (With<BlocksTile>, Without<Monster>)>,
    monsters: Query<(Entity, &Position), With<Monster>>,
) {
    let (player_ent, player_pos, mut sprite) = player.single_mut();

    let (mut x, mut y) = (0, 0);

    // by default we look to the left, so we flip on right move
    if input.just_pressed(KeyCode::ArrowRight)
        || input.just_pressed(KeyCode::KeyL)
        || input.just_pressed(KeyCode::Numpad6)
    {
        x += 1;
        sprite.flip_x = true;
    }
    if input.just_pressed(KeyCode::ArrowLeft)
        || input.just_pressed(KeyCode::KeyH)
        || input.just_pressed(KeyCode::Numpad4)
    {
        x -= 1;
        sprite.flip_x = false;
    }
    if input.just_pressed(KeyCode::ArrowUp)
        || input.just_pressed(KeyCode::KeyK)
        || input.just_pressed(KeyCode::Numpad8)
    {
        y += 1;
    }
    if input.just_pressed(KeyCode::ArrowDown)
        || input.just_pressed(KeyCode::KeyJ)
        || input.just_pressed(KeyCode::Numpad2)
    {
        y -= 1;
    }

    // diagonal movement
    // up, right
    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::KeyU) {
        y += 1;
        x += 1;
        sprite.flip_x = true;
    }
    // down, right
    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::KeyN) {
        y -= 1;
        x += 1;
        sprite.flip_x = true;
    }
    // down, left
    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::KeyB) {
        y -= 1;
        x -= 1;
        sprite.flip_x = false;
    }
    // up, left
    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::KeyY) {
        y += 1;
        x -= 1;
        sprite.flip_x = false;
    }

    // skpping turn
    if input.just_pressed(KeyCode::KeyS) || input.just_pressed(KeyCode::Numpad5) {
        next_state.set(GameState::EnemyTurn);
        return;
    };

    // no movement
    if x == 0 && y == 0 {
        return;
    }

    if let Some((monster_ent, _)) = monsters
        .iter()
        .find(|(_, pos)| **pos == (*player_pos + MovementRequest { x, y }))
    {
        debug!("attacking monster!");
        cmd.entity(player_ent)
            .insert(MeeleeAttackRequest::new(monster_ent));
        next_state.set(GameState::EnemyTurn);
        return;
    }

    if impassable
        .iter()
        .any(|pos| *pos == *player_pos + MovementRequest { x, y })
    {
        return;
    }

    cmd.entity(player_ent).insert(MovementRequest { x, y });
    next_state.set(GameState::EnemyTurn);
}

/// Computes player's current field of vision
fn compute_fov(
    mut player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    blocks_sight: Query<&Position, With<BlocksSight>>,
) {
    let (p_position, mut viewshed) = player_pos.single_mut();
    viewshed.visible_tiles = crate::algorithms::fov::MyVisibility::new(
        |x, y| blocks_sight.iter().any(|pos| pos.x == x && pos.y == y),
        |x, y| euclidean_distance(0, 0, x, y),
    )
    .compute(*p_position, viewshed.visible_range as i32);
}

/// Sets and removes [Visible] component from entities based on player's current [Viewshed]
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

/// With this function we want achiev 3 things:
/// 1. entities that support Fog of War, set alpha of the sprite to [FOW_ALPHA] (mainly walls and floors that player has seen but are not in current Field of View)
/// 2. entities that do not support Fog of War should be set to [Visibility::Hidden]
/// 3. entitties that has been seen before and now enter player's Field of View should have alpha set to 1
///
/// # Arguments
/// * removed - list of entities that we have removed [Visible] from, those are no longer in player's field of view
/// * `fow_sprites` - sprites that are marked with [FogOfWar], those have alpha set to 25% when not in player's Field of View
/// * `hide_visibility` - entities that should be hidden when not in player's Field of View
/// * `visited_sprites` - sprites that player has seen before, are covered by Fog of War and are now again in player's Field of View -> set alpha to 100%
fn apply_fow(
    mut removed: RemovedComponents<Visible>,
    mut fow_sprites: Query<&mut Sprite, (Without<Visible>, With<FogOfWar>)>,
    mut hide_visibility: Query<&mut Visibility, (Without<Visible>, Without<FogOfWar>)>,
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

    fn set_hidden(mut visibility: Mut<Visibility>) {
        *visibility = Visibility::Hidden;
    }

    //
    removed.read().for_each(|entity| {
        fow_sprites.get_mut(entity).map(set_fow_alpha).ok();
        hide_visibility.get_mut(entity).map(set_hidden).ok();
    });

    // sprites that are hidden, but now are in Field of View
    visited_sprites.iter_mut().for_each(set_opaque)
}

fn euclidean_distance(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> i32 {
    let dx = (p1_x - p2_x) as f64;
    let dy = (p1_y - p2_y) as f64;
    ((dx * dx + dy * dy).sqrt()) as i32
}

/// We want camera to follow player, making the player's sprite always to be in the center.
/// This function sets camera's [Transform] to player's [Transform]
fn sync_camera_with_player(
    player_pos: Query<&Transform, With<Player>>,
    mut camera_pos: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera_pos.single_mut().translation = player_pos.single().translation
}

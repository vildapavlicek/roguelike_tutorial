use crate::{
    algorithms::fov::LevelPoint,
    components::{
        requests::{MovementRequest, VisibilityChangeRequest},
        Floor, Impassable, InFov, Player, Position, Revealed, ViewDistance, Wall,
    },
    consts::{PLAYER_Z, SPRITE_SIZE},
};
use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    log::{debug, trace},
    prelude::{
        default, run_once, Changed, Color, Commands, DetectChanges, Entity, IntoSystemConfigs,
        KeyCode, Mut, Or, Plugin, Query, Ref, Res, ResMut, Sprite, SpriteBundle, Startup, Update,
        Vec2, With, Without,
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
                super::sync_camera_with_player,
                (clear_fov, compute_fov, update_visibility)
                    .chain()
                    .run_if(player_moved),
            )
                .chain(),
        );
    }
}

pub(super) fn spawn_player(
    mut cmd: Commands,
    spawn_point: Res<crate::resources::PlayerSpawnPoint>,
    asset_server: Res<AssetServer>,
) {
    let player_sprite = asset_server.load("hooded.png");

    let spawn_point = spawn_point.inner();
    cmd.spawn(SpriteBundle {
        texture: player_sprite,
        ..default()
    })
    .insert(spawn_point)
    // .insert(Position::new(0, 0, PLAYER_Z as i32))
    .insert(crate::components::Player)
    .insert(ViewDistance(4));
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

fn clear_fov(mut cmd: Commands, query: Query<Entity, With<InFov>>) {
    query.iter().for_each(|e| {
        cmd.entity(e).remove::<InFov>();
    });
}

fn compute_fov(
    mut cmd: Commands,
    player_pos: Query<(Ref<Position>, &ViewDistance), With<Player>>,
    walls: Query<&Position, With<Wall>>,
    mut sprites: Query<(Entity, &Position, Option<&Revealed>)>,
) {
    if !player_pos.single().0.is_changed() {
        return;
    }

    let (player_pos, view_distance) = player_pos.single();
    let visible = crate::algorithms::my_fov::MyVisibility::new(
        |x, y| walls.iter().any(|pos| pos.x == x && pos.y == y),
        |x, y| euclidean_distance(0, 0, x, y),
    )
    .compute(*player_pos, view_distance.0 as i32);

    visible.into_iter().for_each(|pos| {
        match sprites.iter_mut().find(|(_, s_pos, _)| **s_pos == pos) {
            Some((entity, _, None)) => {
                cmd.entity(entity).insert(InFov).insert(Revealed);
            }
            Some((entity, _, Some(_))) => {
                cmd.entity(entity).insert(InFov);
            }
            None => (),
        };
    });
}

fn update_visibility(mut visibility: Query<(&mut Visibility, &mut Sprite), With<InFov>>) {
    fn update_visibility((mut visibility, mut sprite): (Mut<Visibility>, Mut<Sprite>)) {
        sprite.color.set_a(1f32);

        if matches!(*visibility, Visibility::Visible) {
            return;
        }

        *visibility = Visibility::Visible;
    }

    visibility.iter_mut().for_each(update_visibility);
}

fn euclidean_distance(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> i32 {
    let dx = (p1_x - p2_x) as f64;
    let dy = (p1_y - p2_y) as f64;
    ((dx * dx + dy * dy).sqrt()) as i32
}

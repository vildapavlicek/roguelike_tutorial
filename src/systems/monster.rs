use crate::{
    ai::{ChasePlayer, CurseAtPlayer, MeeleeAttackPlayer, PlayerInAttackRange, PlayerVisible},
    algorithms::fov::MyVisibility,
    components::{self, BlocksSight, BlocksTile, Monster, Name, Position, Viewshed, Wall},
    resources::SpawnPoints,
    states::GameState,
};
use bevy::{
    app::{PreUpdate, Startup},
    ecs::component,
    log::trace,
    prelude::*,
    utils::HashSet,
};
use big_brain::{pickers::FirstToScore, thinker::Thinker, BigBrainSet};
use rand::Rng;

pub(super) struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            spawn_monsters.run_if(run_once()).after(super::InitSetupSet),
        )
        .add_systems(
            Update,
            (
                compute_fov,
                (
                    (
                        crate::ai::player_visible_scorer_system,
                        crate::ai::player_in_meelee_range_scorer,
                    )
                        .in_set(BigBrainSet::Scorers),
                    (
                        crate::ai::chase_player,
                        crate::ai::meelee_attack_player_action,
                    )
                        .in_set(BigBrainSet::Actions),
                    end_turn,
                )
                    .run_if(in_state(GameState::EnemyTurn))
                    .chain(),
            ),
        );
    }
}

/// This is our monster spawner function, has to be ran after initial setup as we need the [SpawnPoints] resource
pub fn spawn_monsters(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    spawn_points: Res<SpawnPoints>,
) {
    trace!("spawning monsters");
    let goblin = asset_server.load("goblin.png");
    let orc = asset_server.load("orc.png");

    spawn_points
        .monsters
        .iter()
        .enumerate()
        .for_each(|(i, spawn_point)| {
            let (texture, name) = (rand::thread_rng().gen_range(0f32..1f32) > 0.75f32)
                .then(|| (orc.clone(), "Orc"))
                .unwrap_or((goblin.clone(), "Goblin"));
            cmd.spawn((
                SpriteBundle {
                    visibility: bevy::render::view::Visibility::Hidden,
                    texture,
                    ..default()
                },
                *spawn_point,
                Viewshed::new(4),
                Monster,
                BlocksSight,
                Name(format!("{name} {i}")),
                components::bundles::CombatStats::new(16, 4, 1),
                Thinker::build()
                    .picker(FirstToScore { threshold: 0.5 })
                    .when(PlayerInAttackRange, MeeleeAttackPlayer)
                    .when(PlayerVisible, ChasePlayer),
            ));
        })
}

fn compute_fov(
    mut monsters: Query<(&Position, &mut Viewshed), With<Monster>>,
    walls: Query<&Position, With<Wall>>,
) {
    fn compute_and_update_fov<'a>(
        walls: &'a HashSet<&'a Position>,
    ) -> impl FnMut((&'a Position, Mut<'a, Viewshed>)) -> () + 'a {
        return move |(position, mut viewshed): (&'a Position, Mut<'a, Viewshed>)| {
            let visible_tiles = MyVisibility::new(
                |x, y| walls.iter().any(|pos| pos.x == x && pos.y == y),
                |x, y| euclidean_distance(0, 0, x, y),
            )
            .compute(*position, viewshed.visible_range() as i32);
            viewshed.set_visible_tiles(visible_tiles);
        };
    }

    let walls = HashSet::from_iter(walls.into_iter());

    monsters.iter_mut().for_each(compute_and_update_fov(&walls))
}

fn euclidean_distance(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> i32 {
    let dx = (p1_x - p2_x) as f64;
    let dy = (p1_y - p2_y) as f64;
    ((dx * dx + dy * dy).sqrt()) as i32
}

fn end_turn(mut state: ResMut<NextState<GameState>>) {
    trace!("ending monster turn");
    state.set(GameState::PlayerTurn)
}

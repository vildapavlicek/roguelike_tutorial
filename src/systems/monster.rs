use crate::{
    algorithms::fov::MyVisibility,
    components::{Monster, Position, Viewshed, Wall},
    states::GameState,
};
use bevy::{log::trace, prelude::*, utils::HashSet};
use big_brain::BigBrainSet;

pub(super) struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
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
    state.set(GameState::PlayerTurn)
}

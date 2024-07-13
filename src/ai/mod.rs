use crate::components::{
    requests::MeeleeAttackRequest, BlocksTile, Monster, Name, Player, Position, Viewshed,
};
use bevy::{
    log::{debug, error, warn},
    prelude::{Commands, Component, Entity, Mut, Query, With, Without},
    utils::hashbrown::HashSet,
};
use big_brain::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ScorerBuilder, Component)]
pub struct PlayerVisible;

pub fn player_visible_scorer_system(
    viewshed: Query<(&Viewshed, &Position), (With<Monster>, Without<Player>)>,
    mut score_query: Query<(&Actor, &mut Score), With<PlayerVisible>>,
    ppos: Query<&Position, With<Player>>,
) {
    let ppos = ppos.single();
    score_query
        .iter_mut()
        .for_each(|(Actor(actor), mut score)| {
            score.set(
                viewshed
                    .get(*actor)
                    .ok()
                    .map(|(viewshed, _)| viewshed.contains(ppos).then_some(0.6f32))
                    .flatten()
                    .unwrap_or(0f32),
            );
        });
}

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct CurseAtPlayer;

pub fn curse_at_player_action_system(
    mut actors: Query<(&Actor, &mut ActionState), With<CurseAtPlayer>>,
    names: Query<&Name>,
) {
    actors
        .iter_mut()
        .for_each(|(Actor(actor), mut state): (&Actor, Mut<ActionState>)| {
            match *state {
                ActionState::Requested => {
                    let name = names
                        .get(*actor)
                        .map(ToString::to_string)
                        .unwrap_or("Unnamed".into());
                    warn!(%name, "cursing at player");
                    *state = ActionState::Success;
                }
                ActionState::Init => (),
                _ => error!(?state, "unexpected action state"),
            };
        });
}

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct ChasePlayer;

pub fn chase_player(
    mut actors: Query<(&Actor, &mut ActionState), With<ChasePlayer>>,
    blockers: Query<&Position, (With<BlocksTile>, Without<Monster>)>,
    mut mpos: Query<&mut Position, (With<Monster>, Without<BlocksTile>)>,
    ppos: Query<&Position, (With<Player>, Without<Monster>)>,
) {
    let mut impassable = HashSet::new();
    impassable.extend(blockers.into_iter().map(|pos| *pos));

    let finish = *ppos.single();

    let mut finish_positions = HashSet::new();
    finish_positions.extend(finish.possible_successors().into_iter());

    for (Actor(actor), mut action_state) in actors.iter_mut() {
        if !matches!(*action_state, ActionState::Requested) {
            warn!(?action_state, "unexpected action state");
            *action_state = ActionState::Success;
            continue;
        };

        let Ok(monster_pos) = mpos.get(*actor) else {
            continue;
        };

        let mut monster_pos_set = HashSet::new();
        monster_pos_set.extend(mpos.iter().map(|pos| *pos));

        let Some((path, _cost)) = pathfinding::directed::astar::astar(
            monster_pos,
            |p| {
                p.possible_successors()
                    .into_iter()
                    .filter_map(|p| {
                        (p == finish)
                            .then_some((p, 1))
                            .or((!impassable.contains(&p) && !monster_pos_set.contains(&p))
                                .then_some((p, 1)))
                    })
                    .collect::<Vec<(Position, i32)>>()
            },
            |p| p.distance(*monster_pos) / 3,
            |p| finish_positions.contains(p),
        ) else {
            debug!("no path found");
            continue;
        };

        // first [Position] should be start
        match path.get(1) {
            Some(new_pos) => {
                mpos.get_mut(*actor)
                    .map(|mut pos| *pos = *new_pos)
                    .expect("failed to update position, even tho we got it for path resolution");
            }
            None => debug!("path doesn't contain data at index 1"),
        }

        *action_state = ActionState::Success;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ScorerBuilder, Component)]
pub struct PlayerInAttackRange;

/// Checks whether the distance from [Actor] to [Player] is `1` and if so, that means [Monster] is standing next to [Player] and thus can attack it with meelee attack
/// Sets score of `0.1`
pub fn player_in_meelee_range_scorer(
    ppos: Query<&Position, With<Player>>,
    mpos: Query<&Position, (With<Monster>, Without<Player>)>,
    mut score_query: Query<(&Actor, &mut Score), With<PlayerInAttackRange>>,
) {
    let ppos = ppos.single();

    for (Actor(entity), mut score) in score_query.iter_mut() {
        score.set(
            mpos.get(*entity)
                .ok()
                .map(|pos| {
                    //debug!(?ppos, ?pos, ?entity, "player vs monster pos");
                    pos.next_to(ppos).then(|| 1.0)
                })
                .flatten()
                .unwrap_or_default(),
        );
    }
}

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct MeeleeAttackPlayer;

pub fn meelee_attack_player_action(
    mut cmd: Commands,
    mut actors: Query<(&Actor, &mut ActionState), With<MeeleeAttackPlayer>>,
    p_entity: Query<Entity, With<Player>>,
) {
    debug!(
        actors = actors.iter().count(),
        "running meelee attack AI system"
    );
    let p_entity = p_entity.single();

    for (Actor(entity), mut action_state) in actors.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                if let Some(mut ent_commands) = cmd.get_entity(*entity) {
                    ent_commands.insert(MeeleeAttackRequest::new(p_entity));
                    *action_state = ActionState::Success;
                    continue;
                }

                *action_state = ActionState::Failure;
            }
            _ => {
                warn!("unexpected state in meelee system");
                *action_state = ActionState::Success;
            }
        }
    }
}

use crate::components::{Floor, Impassable, Monster, Name, Player, Position, Viewshed, Wall};
use bevy::{
    log::{debug, error, info, trace, warn},
    prelude::{Component, Mut, Query, With, Without},
    utils::hashbrown::HashSet,
};
use big_brain::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ScorerBuilder, Component)]
pub struct PlayerVisible;

pub fn player_visible_system(
    viewshed: Query<&Viewshed, (With<Monster>, Without<Player>)>,
    mut score_query: Query<(&Actor, &mut Score), With<PlayerVisible>>,
    ppos: Query<&Position, With<Player>>,
    names: Query<&Name>,
) {
    let ppos = ppos.single();
    score_query
        .iter_mut()
        .for_each(|(Actor(actor), mut score)| {
            let name = names
                .get(*actor)
                .map(ToOwned::to_owned)
                .unwrap_or(Name::default());

            score.set(
                viewshed
                    .get(*actor)
                    .ok()
                    .map(|viewshed| viewshed.contains(ppos).then_some(1f32))
                    .flatten()
                    .unwrap_or(0f32),
            );
        });
}

#[derive(Debug, Clone, Component, ActionBuilder)]
pub struct CurseAtPlayer;

pub fn curse_at_player_system(
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
    blockers: Query<&Position, (With<Impassable>, Without<Monster>)>,
    mut mpos: Query<(&mut Position, &Name), (With<Monster>, Without<Impassable>)>,
    ppos: Query<&Position, (With<Player>, Without<Monster>)>,
) {
    trace!(actors = actors.iter().count(), "running chase AI system");
    let mut impassable = HashSet::new();
    impassable.extend(blockers.into_iter().map(|pos| *pos));
    let finish = *ppos.single();

    for (Actor(actor), mut action_state) in actors.iter_mut() {
        trace!(?actor, "trying to chase");
        if !matches!(*action_state, ActionState::Requested) {
            warn!(?action_state, "unexpected action state");
            continue;
        };

        let Ok((monster_pos, name)) = mpos.get(*actor) else {
            continue;
        };

        info!(%name, "trying to get path for monster to player");

        let mut monster_pos_set = HashSet::new();
        monster_pos_set.extend(mpos.iter().map(|(pos, _)| *pos));

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
            |p| p == &finish,
        ) else {
            debug!("no path found");
            continue;
        };

        // first [Position] should be start
        match path.get(1) {
            Some(new_pos) => {
                mpos.get_mut(*actor)
                    .map(|((mut pos, _))| *pos = *new_pos)
                    .expect("failed to update position, even tho we got it for path resolution");
            }
            None => debug!("path doesn't contain data at index 1"),
        }

        *action_state = ActionState::Success;
    }
}

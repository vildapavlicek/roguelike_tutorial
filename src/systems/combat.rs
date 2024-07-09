//! Combat related systems
//!
//!

use bevy::prelude::*;

use crate::components::{
    combat::{Defense, Health, Power, SufferDamage},
    requests::MeeleeAttackRequest,
    Name, Player,
};

pub struct CombatSystemPlugin;
impl Plugin for CombatSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (combat_system, (apply_damage, delete_the_dead).chain()),
        );
    }
}

fn combat_system(
    mut cmd: Commands,
    attackers: Query<(Entity, &Name, &MeeleeAttackRequest, &Power)>,
    mut targets: Query<(&Name, &mut SufferDamage, &Health, &Defense)>,
) {
    fn process_attack(
        mut suffer_damage: Mut<SufferDamage>,
        health: &Health,
        power: &Power,
        defense: &Defense,
        attacker_name: &Name,
        target_name: &Name,
    ) {
        if health.current < health.min {
            return;
        }

        let damage = i32::max(0, power.0 - defense.0);

        if damage == 0 {
            debug!(%attacker_name, %target_name, "failed to apply damage to target");
        }

        debug!(%attacker_name, %target_name, %damage, "attack success");
        suffer_damage.add_damage(damage);
    }

    trace!(attackers = %attackers.iter().count(), "processing combat");
    for (entity, attacker_name, MeeleeAttackRequest { target }, power) in attackers.into_iter() {
        debug!(%attacker_name, ?target, "trying to attack");
        match targets.get_mut(*target) {
            Ok((target_name, suffer_damage, health, defense)) => {
                process_attack(
                    suffer_damage,
                    health,
                    power,
                    defense,
                    attacker_name,
                    target_name,
                );
            }
            Err(err) => error!(%err, "failed to attack target"),
        }
        cmd.entity(entity).remove::<MeeleeAttackRequest>();
    }
}

fn apply_damage(mut query: Query<(&mut Health, &mut SufferDamage)>) {
    fn apply_damage((mut health, mut suffer_damage): (Mut<Health>, Mut<SufferDamage>)) {
        health.take_damage(suffer_damage.drain().sum())
    }

    query.iter_mut().for_each(apply_damage);
}

fn delete_the_dead(mut cmd: Commands, query: Query<(Entity, &Health), Without<Player>>) {
    query.iter().for_each(|(entity, health)| {
        if health.is_dead() {
            cmd.entity(entity).despawn();
        }
    });
}

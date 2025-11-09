#![warn(clippy::pedantic)]

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[read_component(WantsToAttact)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, command: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttact)>::query();

    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<_>>();

    victims.iter().for_each(|(message, victim)| {
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                command.remove(*victim);
            }
            println!("Health after attack:{}", health.current);
        }
        command.remove(*message);
    });
}

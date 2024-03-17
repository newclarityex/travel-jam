use crate::core::{
    environment::{Catnip, Yarn},
    items::Inventory,
    player::Player,
};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

pub fn update_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, Entity)>,
    mut yarn_query: Query<(&mut Yarn, Entity)>,
    mut catnip_query: Query<(&mut Catnip, Entity)>,
    mut inventory: ResMut<Inventory>,
) {
    let Ok((mut player, player_entity)) = player_query.get_single_mut() else {
        eprintln!("Player missing!");
        return;
    };
    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(entity_a, entity_b, flags) => {
                let other_entity = if player_entity == entity_a {
                    entity_b
                } else {
                    entity_a
                };

                if !flags.contains(CollisionEventFlags::SENSOR) {
                    player.collisions.insert(other_entity);
                }

                if yarn_query.contains(other_entity) {
                    inventory.yarn += 1;
                    commands.entity(other_entity).despawn();
                }

                if catnip_query.contains(other_entity) {
                    commands.entity(other_entity).despawn();
                }
            }
            CollisionEvent::Stopped(entity_a, entity_b, flags) => {
                let other_entity = if player_entity == entity_a {
                    entity_b
                } else {
                    entity_a
                };

                if !flags.contains(CollisionEventFlags::SENSOR) {
                    player.collisions.remove(&other_entity);
                }
            }
        }
    }
}

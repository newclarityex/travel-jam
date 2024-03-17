use crate::core::{
    environment::{BigYarn, Catnip, Yarn},
    items::Inventory,
    player::Player,
};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

const CATNIP_BOOST: f32 = 1500.;
const CATNIP_ANGLE: Vec2 = Vec2::X;
const YARN_SCORE: i32 = 100;
const BIG_YARN_SCORE: i32 = 500;
const CATNIP_SCORE: i32 = 250;

pub fn update_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, Entity, &mut ExternalImpulse, &Transform)>,
    mut yarn_query: Query<(&mut Yarn, Entity)>,
    mut big_yarn_query: Query<(&mut BigYarn, Entity)>,
    mut catnip_query: Query<(&mut Catnip, Entity)>,
    mut inventory: ResMut<Inventory>,
) {
    let Ok((mut player, player_entity, mut player_impulse, player_transform)) =
        player_query.get_single_mut()
    else {
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
                    player.current_yarn_collected += 1;
                    player.score += YARN_SCORE;
                    commands.entity(other_entity).despawn();
                }

                if big_yarn_query.contains(other_entity) {
                    inventory.yarn += 5;
                    player.current_yarn_collected += 5;
                    player.score += BIG_YARN_SCORE;
                    commands.entity(other_entity).despawn();
                }

                if catnip_query.contains(other_entity) {
                    let axis_angle = player_transform.rotation.to_axis_angle();
                    let force_angle = axis_angle.0.z * axis_angle.1;
                    player_impulse.impulse = Vec2::from_angle(force_angle) * CATNIP_BOOST;

                    player.current_catnip_collected += 1;
                    player.score += CATNIP_SCORE;
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

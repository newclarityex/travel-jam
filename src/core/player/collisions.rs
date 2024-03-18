use crate::core::{
    animations::{AnimationCompleteEvent, AnimationData, AnimationsManager},
    audio::SFXChannel,
    environment::{BigYarn, Catnip, Yarn},
    items::Inventory,
    player::Player,
    CleanupEntity,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::{prelude::*, rapier::geometry::CollisionEventFlags};

use super::CollectionHitbox;

const CATNIP_BOOST: f32 = 1500.;
const CATNIP_ANGLE: Vec2 = Vec2::X;
const YARN_SCORE: i32 = 100;
const BIG_YARN_SCORE: i32 = 500;
const CATNIP_SCORE: i32 = 250;

pub fn update_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Player, Entity, &mut ExternalImpulse, &Transform)>,
    collection_hitbox_query: Query<Entity, With<CollectionHitbox>>,
    yarn_query: Query<(&Transform), With<Yarn>>,
    big_yarn_query: Query<(&Transform), With<BigYarn>>,
    catnip_query: Query<(&Transform), With<Catnip>>,
    mut inventory: ResMut<Inventory>,
    asset_server: Res<AssetServer>,
    sfx_channel: Res<AudioChannel<SFXChannel>>,
) {
    let Ok((mut player, player_entity, mut player_impulse, player_transform)) =
        player_query.get_single_mut()
    else {
        eprintln!("Player missing!");
        return;
    };

    let Ok(collection_hitbox_query) = collection_hitbox_query.get_single() else {
        eprintln!("Player hitbox missing!");
        return;
    };

    for collision_event in collision_events.read() {
        match *collision_event {
            CollisionEvent::Started(entity_a, entity_b, flags) => {
                let other_entity =
                    if player_entity == entity_a || collection_hitbox_query == entity_a {
                        entity_b
                    } else {
                        entity_a
                    };

                if !flags.contains(CollisionEventFlags::SENSOR) {
                    player.collisions.insert(other_entity);
                }

                let yarn = yarn_query.get(other_entity);
                if let Ok(yarn_transform) = yarn {
                    inventory.yarn += 1;
                    player.current_yarn_collected += 1;
                    player.score += YARN_SCORE;
                    commands.entity(other_entity).despawn();
                    sfx_channel.play(asset_server.load("audio/sfx/coin.wav"));
                }

                if big_yarn_query.contains(other_entity) {
                    inventory.yarn += 5;
                    player.current_yarn_collected += 5;
                    player.score += BIG_YARN_SCORE;
                    commands.entity(other_entity).despawn();
                    sfx_channel.play(asset_server.load("audio/sfx/coin.wav"));
                }

                if catnip_query.contains(other_entity) {
                    let axis_angle = player_transform.rotation.to_axis_angle();
                    let force_angle = axis_angle.0.z * axis_angle.1;
                    player_impulse.impulse = Vec2::from_angle(force_angle) * CATNIP_BOOST;

                    player.current_catnip_collected += 1;
                    player.score += CATNIP_SCORE;
                    commands.entity(other_entity).despawn();
                    sfx_channel.play(asset_server.load("audio/sfx/coin.wav"));
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

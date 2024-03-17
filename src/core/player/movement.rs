use super::{Player, PlayerSprite, PlayerState};
use crate::{
    core::{animations::AnimationsManager, items::VehicleTextures, GameStage},
    MainCamera,
};
use bevy::{ecs::entity, prelude::*};
use bevy_parallax::ParallaxMoveEvent;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub fn handle_pushing(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut ExternalForce, &mut Transform)>,
    time: Res<Time>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let accelerating = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let Ok((mut player, mut force, transform)) = player_query.get_single_mut() else {
        eprintln!("Missing player!");
        return;
    };

    if accelerating {
        let axis_angle = transform.rotation.to_axis_angle();
        let force_angle = axis_angle.0.z * axis_angle.1;
        force.force = Vec2::from_angle(force_angle) * player.push_force;
    } else {
        force.force = Vec2::ZERO;
    }

    if transform.translation.x > 0. {
        next_player_state.set(PlayerState::Sliding);
    }
}

pub fn stop_pushing(mut player_query: Query<&mut ExternalForce, With<Player>>) {
    let Ok(mut force) = player_query.get_single_mut() else {
        eprintln!("Missing player!");
        return;
    };

    force.force = Vec2::ZERO;
}

pub fn handle_sliding(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Player,
        &mut Velocity,
        &mut ExternalForce,
        &mut ExternalImpulse,
    )>,
    mut next_game_stage: ResMut<NextState<GameStage>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let Ok((player, mut velocity, mut force, mut impulse)) = player_query.get_single_mut() else {
        eprintln!("Missing player!");
        return;
    };

    if player.collisions.len() > 0 {
        if keys.just_pressed(KeyCode::Space) {
            velocity.linvel.y = player.jump_vel;
        }
    } else {
        let mut rotation_direction = 0.;
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            rotation_direction += 1.;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            rotation_direction -= 1.;
        }

        force.torque = rotation_direction * player.lean_force * time.delta_seconds();
    }

    if velocity.linvel.x < 0.5 {
        next_game_stage.set(GameStage::Stopped);
        next_player_state.set(PlayerState::Pushing);
    }
}

const CAMERA_Y_MIN: f32 = 56.;

pub fn update_camera_position(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(Entity, &mut Transform), (With<MainCamera>, Without<Player>)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        eprintln!("Missing Player!");
        return;
    };

    let Ok((camera_entity, mut camera_transform)) = camera_query.get_single_mut() else {
        eprintln!("Missing Camera!");
        return;
    };

    let mut target_pos = player_transform.translation;
    target_pos.y = target_pos.y.max(CAMERA_Y_MIN);

    let distance_diff = target_pos - camera_transform.translation;

    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: Vec2::ZERO.lerp(distance_diff.xy(), 10. * time.delta_seconds()),
        camera: camera_entity,
    });
}

pub fn update_hide_state(
    player_query: Query<&Transform, With<Player>>,
    mut player_sprite_query: Query<&mut Visibility, With<PlayerSprite>>,
    mut vehicle_query: Query<(&VehicleTextures, &mut Handle<Image>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok((vehicle_textures, mut vehicle_texture)) = vehicle_query.get_single_mut() else {
        return;
    };
    let Ok(mut player_visibility) = player_sprite_query.get_single_mut() else {
        return;
    };

    let angle = player_transform.rotation.to_axis_angle().1;
    if angle > (PI / 2.) - 0.05 && angle < (3. / 2. * PI) + 0.05 {
        *player_visibility = Visibility::Hidden;
        *vehicle_texture = vehicle_textures.closed_texture.clone();
    } else {
        *player_visibility = Visibility::Visible;
        *vehicle_texture = vehicle_textures.open_texture.clone();
    }
}

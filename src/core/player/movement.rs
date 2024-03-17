use super::{Player, PlayerSprite, PlayerState};
use crate::core::{
    animations::AnimationsManager, camera::MainCamera, items::VehicleTextures, GameStage,
};
use bevy::{ecs::entity, prelude::*};
use bevy_parallax::ParallaxMoveEvent;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub fn handle_pushing(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(
        &mut Player,
        &mut ExternalForce,
        &mut Transform,
        &mut Damping,
    )>,
    time: Res<Time>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let accelerating = keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let Ok((mut player, mut force, transform, mut damping)) = player_query.get_single_mut() else {
        eprintln!("Missing player!");
        return;
    };

    damping.angular_damping = 50.;

    if accelerating {
        let axis_angle = transform.rotation.to_axis_angle();
        let force_angle = axis_angle.0.z * axis_angle.1;
        force.force += Vec2::from_angle(force_angle) * player.push_force;
        // force.force += Vec2::from_angle(-PI / 4.) * player.push_force;
    }

    if transform.translation.x > 0. {
        next_player_state.set(PlayerState::Sliding);
    }
}

pub fn reset_force(mut player_query: Query<&mut ExternalForce>) {
    let Ok(mut force) = player_query.get_single_mut() else {
        eprintln!("Missing player!");
        return;
    };

    force.force = Vec2::ZERO;
    force.torque = 0.;
}

pub fn handle_sliding(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Player,
        &mut Velocity,
        &mut ExternalForce,
        &mut ExternalImpulse,
        &Transform,
        &ReadMassProperties,
        &mut Damping,
    )>,
    mut next_game_stage: ResMut<NextState<GameStage>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let Ok((player, mut velocity, mut force, mut impulse, transform, mass, mut damping)) =
        player_query.get_single_mut()
    else {
        eprintln!("Missing player!");
        return;
    };

    damping.angular_damping = 5.;

    if player.collisions.len() > 0 {
        if keys.just_pressed(KeyCode::Space) {
            velocity.linvel.y = player.jump_vel;
        }
    }

    let mut rotation_direction = 0.;
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        rotation_direction += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        rotation_direction -= 1.;
    }

    force.torque += rotation_direction * player.lean_force * time.delta_seconds();

    let axis_angle = transform.rotation.to_axis_angle();
    let player_angle = axis_angle.0.z * axis_angle.1;

    force.torque += (rotation_direction - player_angle) * player.float_val;

    if let Some(max_fall_vel) = player.max_fall_vel {
        velocity.linvel.y = velocity.linvel.y.max(max_fall_vel);
    }

    if let Some(gliding_scale) = player.gliding_scale {
        let glider_angle = player_angle;
        let vel_angle = velocity.linvel.to_angle();

        let force_factor = (glider_angle - vel_angle).sin();
        let gliding_direction = Vec2::from_angle(glider_angle + PI / 2.);
        let gliding_force = mass.get().mass * force_factor * velocity.linvel.length();

        force.force += gliding_force * gliding_direction * gliding_scale;
    }

    if velocity.linvel.length() < 1. {
        next_game_stage.set(GameStage::Stopped);
        next_player_state.set(PlayerState::Pushing);
    }
}

// const CAMERA_Y_MIN: f32 = 56.;
const CAMERA_Y_MIN: f32 = 108.;

pub fn update_camera_position(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut MainCamera)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        eprintln!("Missing Player!");
        return;
    };

    let Ok(mut camera) = camera_query.get_single_mut() else {
        eprintln!("Missing Camera!");
        return;
    };

    let mut target_pos = player_transform.translation;
    target_pos.y = target_pos.y.max(CAMERA_Y_MIN);

    camera.target_position = target_pos.xy();
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

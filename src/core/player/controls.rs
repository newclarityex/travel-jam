use crate::{core::animations::AnimationsManager, PauseState, SettingsState};
use bevy::prelude::*;

use super::Player;

pub fn handle_pause_input(
    state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            PauseState::Paused => {
                next_pause_state.set(PauseState::Running);
                next_settings_state.set(SettingsState::Closed);
            }
            PauseState::Running => {
                next_pause_state.set(PauseState::Paused);
            }
        }
    }
}

pub fn handle_move_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut AnimationsManager,
        &mut Sprite,
    )>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let normalized_direction = direction.normalize_or_zero();

    let Ok((mut player, mut transform, mut animations_manager, mut sprite)) =
        player_query.get_single_mut()
    else {
        return;
    };

    if direction != Vec2::ZERO {
        animations_manager.time_scale = 1.;
    } else {
        animations_manager.time_scale = 0.;
    }

    if direction.x < 0. {
        sprite.flip_x = true;
    } else if direction.x > 0. {
        sprite.flip_x = false;
    }

    let new_velocity = player.velocity.lerp(
        direction * player.max_speed,
        1. / player.acceleration * time.delta_seconds(),
    );

    player.velocity = new_velocity;
    player.position += new_velocity * time.delta_seconds();
    transform.translation = player.position.extend(0.);
}

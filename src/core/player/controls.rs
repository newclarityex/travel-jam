use crate::PauseState;
use bevy::prelude::*;

pub fn handle_pause_input(
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            PauseState::Paused => {
                next_state.set(PauseState::Running);
            }
            PauseState::Running => {
                next_state.set(PauseState::Paused);
            }
        }
    }
}

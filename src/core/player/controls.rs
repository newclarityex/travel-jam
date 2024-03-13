use crate::{PauseState, SettingsState};
use bevy::prelude::*;

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

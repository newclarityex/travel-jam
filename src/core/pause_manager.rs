use crate::core::SettingsState;
use bevy::prelude::*;
use bevy_rapier2d::plugin::{RapierConfiguration, TimestepMode};

use super::GameState;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    Paused,
    Running,
}

pub struct PauseManagerPlugin;
impl Plugin for PauseManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PauseState::Running)
            .add_systems(Update, handle_pause.run_if(in_state(GameState::Game)))
            .add_systems(OnEnter(PauseState::Paused), on_pause)
            .add_systems(OnEnter(PauseState::Running), on_unpause);
    }
}

fn handle_pause(
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

fn on_pause(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: 0.,
        substeps: 1,
    };
}

fn on_unpause(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: 1.,
        substeps: 1,
    };
}

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum DebugState {
    Open,
    Closed,
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugState::Closed)
            .add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugState::Open)))
            .add_systems(Update, handle_debug_input);
    }
}

fn handle_debug_input(
    state: Res<State<DebugState>>,
    mut next_state: ResMut<NextState<DebugState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F8) {
        match state.get() {
            DebugState::Open => {
                next_state.set(DebugState::Closed);
            }
            DebugState::Closed => {
                next_state.set(DebugState::Open);
            }
        }
    }
}

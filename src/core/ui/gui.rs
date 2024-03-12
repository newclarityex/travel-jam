use crate::PauseState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{Align2, RichText};

pub fn gui_system(
    mut contexts: EguiContexts,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    egui::Area::new("area")
        .anchor(Align2::LEFT_TOP, [0., 0.])
        .show(contexts.ctx_mut(), |ui| match state.get() {
            PauseState::Paused => {
                if ui
                    .button(RichText::new("Unpause").strong().size(32.))
                    .clicked()
                {
                    next_state.set(PauseState::Running);
                }
            }
            PauseState::Running => {
                if ui
                    .button(RichText::new("Pause").strong().size(32.))
                    .clicked()
                {
                    next_state.set(PauseState::Paused);
                }
            }
        });
}

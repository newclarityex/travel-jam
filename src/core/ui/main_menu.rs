use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{Align2, RichText};

pub fn main_menu_system(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    egui::Area::new("area")
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Travel <3");
            if ui
                .button(RichText::new("Start Game").strong().size(32.))
                .clicked()
            {
                next_state.set(GameState::Game);
            }
        });
}

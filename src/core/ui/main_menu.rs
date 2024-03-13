use crate::{GameState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, util::undoer::Settings},
    EguiContexts,
};
use egui::{Align2, RichText};

pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
) {
    egui::Area::new("main_menu")
        .anchor(Align2::LEFT_CENTER, [32., 0.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(RichText::new("Travel <3").size(48.));
            ui.add_space(32.);

            if ui
                .button(RichText::new("Start Game").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_game_state.set(GameState::Game);
            }
            ui.add_space(8.);

            if ui
                .button(RichText::new("Settings").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_settings_state.set(SettingsState::Open);
            }
        });
}

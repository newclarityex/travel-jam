use crate::{GameState, PauseState, SettingsState};
use bevy::prelude::*;
use bevy_egui::egui::util::undoer::Settings;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, Color32, Painter, Pos2, Rect, Rounding, Stroke, Style,
        TextureOptions, Vec2,
    },
    EguiContexts,
};
use egui::{Align2, RichText};

pub fn pause_menu_system(
    mut contexts: EguiContexts,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
) {
    egui::Area::new("pause_menu")
        .anchor(Align2::LEFT_CENTER, [32., 0.])
        .order(egui::Order::Middle)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(RichText::new("Paused").size(48.));
            ui.add_space(32.);
            if ui
                .button(RichText::new("Resume").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_pause_state.set(PauseState::Running);
            }
            ui.add_space(8.);
            if ui
                .button(RichText::new("Settings").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_settings_state.set(SettingsState::Open);
            }
            ui.add_space(8.);
            if ui
                .button(RichText::new("Exit").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_pause_state.set(PauseState::Running);
                next_game_state.set(GameState::MainMenu);
            }
        });
}

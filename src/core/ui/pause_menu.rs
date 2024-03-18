use crate::core::{pause_manager::PauseState, GameStage, GameState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, util::undoer::Settings, Align2, Color32, Painter, Pos2, Rect,
        RichText, Rounding, Stroke, Style, TextureOptions, Vec2,
    },
    EguiContexts,
};

use super::IngameMenu;

pub fn pause_menu_system(
    mut contexts: EguiContexts,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
    mut next_ingame_state: ResMut<NextState<IngameMenu>>,
    mut next_game_stage: ResMut<NextState<GameStage>>,
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
                next_ingame_state.set(IngameMenu::Stats);
                next_game_stage.set(GameStage::Sledding);
            }
        });
}

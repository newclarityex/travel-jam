use crate::core::{camera::MainCamera, GameState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, util::undoer::Settings, Align2, Color32, Image, RichText},
    EguiContexts,
};

pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = camera_query.get_single_mut() else {
        eprintln!("Missing Camera");
        return;
    };

    egui::Area::new("main_menu")
        .anchor(Align2::LEFT_CENTER, [32., 0.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            let title_image = Image::new(egui::include_image!("../../../assets/ui/title.png"))
                .fit_to_original_size(0.8);
            ui.add(title_image);
            ui.add_space(64.);

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

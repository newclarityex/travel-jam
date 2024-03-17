use crate::core::{camera::MainCamera, GameState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, util::undoer::Settings, Align2, Color32, RichText},
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
            ui.heading(
                RichText::new("When Cats Fly")
                    .size(48.)
                    .italics()
                    .color(Color32::from_rgb(255, 120, 120))
                    .background_color(Color32::from_black_alpha(200)),
            );
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

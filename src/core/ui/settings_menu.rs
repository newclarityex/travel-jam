use crate::{PauseState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, Color32, Painter, Pos2, Rect, Rounding, Stroke, Style,
        TextureOptions, Vec2,
    },
    EguiContexts,
};
use egui::{Align2, RichText};

pub fn settings_menu_system(
    mut contexts: EguiContexts,
    state: Res<State<SettingsState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
) {
    egui::Area::new("settings_menu")
        .anchor(Align2::LEFT_CENTER, [32., 0.])
        .order(egui::Order::Foreground)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(RichText::new("Settings").strong().size(48.));
            ui.add_space(32.);
            if ui
                .button(RichText::new("Back").strong().size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_settings_state.set(SettingsState::Closed);
            }
        });
}

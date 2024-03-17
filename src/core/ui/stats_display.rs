use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, util::undoer::Settings, Align2, Color32, Margin, Painter, Pos2,
        Rect, RichText, Rounding, Stroke, Style, TextureOptions, Vec2,
    },
    EguiContexts,
};

use crate::core::GameStage;

use super::IngameMenu;

pub fn stats_display_system(
    mut contexts: EguiContexts,
    mut next_ingame_menu_state: ResMut<NextState<IngameMenu>>,
) {
    egui::Area::new("stats_display")
        .anchor(Align2::CENTER_CENTER, [32., 0.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            egui::Frame::none()
                .inner_margin(Margin::same(8.))
                .outer_margin(Margin::same(0.))
                .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 127))
                .show(ui, |ui| {
                    ui.heading(RichText::new("Stopped").size(48.));
                    ui.add_space(32.);

                    if ui
                        .button(RichText::new("Finish").size(32.))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        next_ingame_menu_state.set(IngameMenu::Shop)
                    }
                });
        });
}

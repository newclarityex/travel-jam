use crate::core::player::Player;
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, util::undoer::Settings, Align2, Color32, Grid, Layout, Margin,
        Painter, Pos2, Rect, RichText, Rounding, Stroke, Style, TextureOptions, Vec2,
    },
    EguiContexts,
};

use crate::core::GameStage;

use super::IngameMenu;

pub fn stats_display_system(
    mut contexts: EguiContexts,
    mut next_ingame_menu_state: ResMut<NextState<IngameMenu>>,
    player_query: Query<(&Player, &Transform)>,
) {
    let Ok((player, player_transform)) = player_query.get_single() else {
        return;
    };

    egui::Area::new("stats_display")
        .anchor(Align2::CENTER_CENTER, [32., 0.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            egui::Frame::none()
                .inner_margin(Margin::symmetric(16., 8.))
                .outer_margin(Margin::same(0.))
                .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 200))
                .show(ui, |ui| {
                    ui.set_width(256. + 16. * 2.);
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Complete!").size(32.));
                        ui.add_space(16.);
                        Grid::new("stats_grid").min_col_width(128.).show(ui, |ui| {
                            let distance = (player_transform.translation.x.max(0.) / 10.) as i32;
                            ui.label(RichText::new("Distance: ").size(18.));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("{} m", distance)).size(18.));
                            });
                            ui.end_row();
                            ui.label(RichText::new("Yarn Collected: ").size(18.));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{}", player.current_yarn_collected))
                                        .size(18.),
                                );
                            });
                            ui.end_row();
                            ui.label(RichText::new("Catnip Collected: ").size(18.));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!("{}", player.current_catnip_collected))
                                        .size(18.),
                                );
                            });
                            ui.end_row();
                            ui.label(RichText::new("Score: ").size(18.));
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("{}", player.score)).size(18.));
                            });
                        });
                        ui.add_space(16.);
                        if ui
                            .button(RichText::new("Finish").size(16.))
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            next_ingame_menu_state.set(IngameMenu::Shop)
                        }
                    });
                });
        });
}

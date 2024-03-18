use crate::core::{items::Inventory, pause_manager::PauseState, player::Player, GameState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, Align, Align2, Area, Color32, Grid, Image, Label, Layout, Margin,
        Painter, Pos2, Rect, RichText, Rounding, Stroke, Style, TextureOptions, Vec2,
    },
    EguiContexts,
};
use bevy_rapier2d::prelude::Velocity;

pub fn gui_system(
    mut contexts: EguiContexts,
    // state: Res<State<PauseState>>,
    // mut next_pause_state: ResMut<NextState<PauseState>>,
    inventory: Res<Inventory>,
    player_query: Query<(&Transform, &Velocity), With<Player>>,
) {
    let Ok((player_transform, player_velocity)) = player_query.get_single() else {
        eprintln!("Player not found");
        return;
    };

    Area::new("stats_gui")
        .anchor(Align2::RIGHT_TOP, [-6., 6.])
        .order(egui::Order::Background)
        .show(contexts.ctx_mut(), |ui| {
            egui::Frame::none()
                .inner_margin(Margin::same(8.))
                .outer_margin(Margin::same(0.))
                .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 127))
                .show(ui, |ui| {
                    Grid::new("stats_grid").min_col_width(128.).show(ui, |ui| {
                        let distance = (player_transform.translation.x.max(0.) / 10.) as i32;
                        ui.label(
                            RichText::new("Distance: ")
                                .size(32.)
                                .strong()
                                .family(egui::FontFamily::Monospace)
                                .color(Color32::WHITE),
                        );
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{} m", distance))
                                    .size(32.)
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(Color32::WHITE),
                            );
                        });
                        ui.end_row();
                        let altitude = (player_transform.translation.y.max(0.) / 10.) as i32;
                        ui.label(
                            RichText::new("Altitude: ")
                                .size(32.)
                                .strong()
                                .family(egui::FontFamily::Monospace)
                                .color(Color32::WHITE),
                        );
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{} m", altitude))
                                    .size(32.)
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(Color32::WHITE),
                            );
                        });
                        ui.end_row();
                        let speed = (player_velocity.linvel.length() / 10.) as i32;
                        ui.label(
                            RichText::new("Speed: ")
                                .size(32.)
                                .strong()
                                .family(egui::FontFamily::Monospace)
                                .color(Color32::WHITE),
                        );
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{} m/s", speed))
                                    .size(32.)
                                    .strong()
                                    .family(egui::FontFamily::Monospace)
                                    .color(Color32::WHITE),
                            );
                        });
                        ui.end_row();
                    });
                });
        });
    // let pause_icon = egui::Image::new(egui::include_image!(
    //     "../../../assets/ui/pause_button_scaled.png"
    // ))
    // .texture_options(TextureOptions::NEAREST)
    // .fit_to_original_size(1.);
    // egui::Area::new("pause_btn")
    //     .anchor(Align2::RIGHT_TOP, [-6., 6.])
    //     .order(egui::Order::Background)
    //     .show(contexts.ctx_mut(), |ui| {
    //         if ui
    //             .add(egui::Button::image(pause_icon).fill(Color32::TRANSPARENT))
    //             .on_hover_cursor(egui::CursorIcon::PointingHand)
    //             .clicked()
    //         {
    //             match state {
    //                 PauseState::Paused => {
    //                     next_pause_state.set(PauseState::Running);
    //                 }
    //                 PauseState::Running => {
    //                     next_pause_state.set(PauseState::Paused);
    //                 }
    //             };
    //         }
    //     });
}

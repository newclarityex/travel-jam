use crate::core::{items::Inventory, pause_manager::PauseState, player::Player, GameState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, Align, Align2, Area, Color32, Image, Label, Layout, Margin,
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
                    let distance = (player_transform.translation.x.max(0.) / 100.) as i32;
                    ui.add(
                        Label::new(
                            RichText::new(format!("{} m", distance))
                                .size(32.)
                                .strong()
                                .family(egui::FontFamily::Monospace),
                        )
                        .wrap(false),
                    );
                    let velocity = (player_velocity.linvel.length() / 100.) as i32;
                    ui.add(
                        Label::new(
                            RichText::new(format!("{} m/s", velocity))
                                .size(32.)
                                .strong()
                                .family(egui::FontFamily::Monospace),
                        )
                        .wrap(false),
                    );
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

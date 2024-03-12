use crate::{GameState, PauseState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, epaint::RectShape, Color32, Painter, Pos2, Rect, Rounding, Stroke, Style,
        TextureOptions, Vec2,
    },
    EguiContexts,
};
use egui::{Align2, RichText};

pub fn gui_system(
    mut contexts: EguiContexts,
    state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let state = state.get();
    let pause_icon = egui::Image::new(egui::include_image!(
        "../../../assets/ui/pause_button_scaled.png"
    ))
    .texture_options(TextureOptions::NEAREST)
    .fit_to_original_size(1.);
    egui::Area::new("pause_btn")
        .anchor(Align2::RIGHT_TOP, [-6., 6.])
        .order(egui::Order::Middle)
        .show(contexts.ctx_mut(), |ui| {
            if ui
                .add(egui::Button::image(pause_icon).fill(Color32::TRANSPARENT))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                match state {
                    PauseState::Paused => {
                        next_pause_state.set(PauseState::Running);
                    }
                    PauseState::Running => {
                        next_pause_state.set(PauseState::Paused);
                    }
                };
            }
        });

    if *state == PauseState::Paused {
        egui::Area::new("pause_menu_background")
            .anchor(Align2::LEFT_TOP, [0., 0.])
            .order(egui::Order::Foreground)
            .show(contexts.ctx_mut(), |ui| {
                if ui
                    .add(
                        egui::Button::new("")
                            .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 127))
                            .stroke(Stroke::NONE)
                            .min_size(ui.available_size()),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    next_pause_state.set(PauseState::Running);
                }
            });

        egui::Area::new("pause_menu")
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .order(egui::Order::Tooltip)
            .show(contexts.ctx_mut(), |ui| {
                let area_rect = Rect::from_center_size(Pos2::ZERO, Vec2::splat(4.));

                ui.painter_at(area_rect).add(RectShape::filled(
                    area_rect,
                    Rounding::same(0.),
                    Color32::RED,
                ));
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Paused").strong().size(32.));
                    if ui
                        .button(RichText::new("Unpause").strong().size(32.))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        next_pause_state.set(PauseState::Running);
                    }
                    if ui
                        .button(RichText::new("Exit").strong().size(32.))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        next_pause_state.set(PauseState::Running);
                        next_game_state.set(GameState::MainMenu);
                    }
                });
            });
    }
}

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
) {
    let state = state.get();
    let pause_icon = egui::Image::new(egui::include_image!(
        "../../../assets/ui/pause_button_scaled.png"
    ))
    .texture_options(TextureOptions::NEAREST)
    .fit_to_original_size(1.);
    egui::Area::new("pause_btn")
        .anchor(Align2::RIGHT_TOP, [-6., 6.])
        .order(egui::Order::Background)
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
}

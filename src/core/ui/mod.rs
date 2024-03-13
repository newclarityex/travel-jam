use crate::{GameState, PauseState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        epaint::RectShape, Align, Align2, Area, Color32, FontData, FontDefinitions, FontFamily,
        Frame, Id, LayerId, Order, Painter, Rounding, Sense, Vec2,
    },
    EguiContexts,
};
use egui_extras::install_image_loaders;

mod gui;
mod main_menu;
mod pause_menu;
mod settings_menu;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                main_menu::main_menu_system.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(SettingsState::Closed)),
                ),
            )
            .add_systems(
                Update,
                pause_menu::pause_menu_system
                    .run_if(in_state(PauseState::Paused).and_then(in_state(SettingsState::Closed))),
            )
            .add_systems(
                Update,
                settings_menu::settings_menu_system.run_if(in_state(SettingsState::Open)),
            )
            .add_systems(
                Update,
                darken_bg
                    .run_if(in_state(PauseState::Paused).or_else(in_state(SettingsState::Open))),
            )
            .add_systems(Update, gui::gui_system.run_if(in_state(GameState::Game)));
    }
}

fn setup(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();
    let font_data = FontData::from_static(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/Oxygen-Bold.ttf"
    )));

    fonts
        .font_data
        .insert("quicksand_regular".to_owned(), font_data);

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "quicksand_regular".to_owned());

    let egui_ctx = contexts.ctx_mut();
    egui_ctx.set_fonts(fonts);

    egui_ctx.style_mut(|style| {
        style.spacing.button_padding = Vec2::new(16., 8.);
        style.spacing.item_spacing = Vec2::splat(8.);
        style.visuals.override_text_color = Some(Color32::WHITE);
    });

    install_image_loaders(&egui_ctx);
}

fn darken_bg(mut contexts: EguiContexts) {
    let rect = contexts.ctx_mut().available_rect();

    // Consume click events
    Area::new("darken_bg_area")
        .order(Order::Background)
        .movable(false)
        .anchor(Align2::LEFT_TOP, [0., 0.])
        .show(contexts.ctx_mut(), |ui| {
            let (_, painter) = ui.allocate_painter(rect.size(), Sense::click_and_drag());
            painter.add(RectShape::filled(
                rect,
                Rounding::ZERO,
                Color32::from_black_alpha(127),
            ));
        });
}

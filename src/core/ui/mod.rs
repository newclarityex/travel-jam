use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::{FontData, FontDefinitions, FontFamily};
use bevy_egui::EguiContexts;
use egui_extras::install_image_loaders;

mod gui;
mod main_menu;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            main_menu::main_menu_system.run_if(in_state(GameState::MainMenu)),
        );
        app.add_systems(Update, gui::gui_system.run_if(in_state(GameState::Game)));
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

    install_image_loaders(&egui_ctx);
}

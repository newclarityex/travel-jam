use crate::core::{pause_manager::PauseState, GameStage, GameState, SettingsState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        epaint::RectShape,
        style::{WidgetVisuals, Widgets},
        Align, Align2, Area, Color32, FontData, FontDefinitions, FontFamily, Frame, Id, LayerId,
        Order, Painter, Rounding, Sense, Stroke, Vec2,
    },
    EguiContexts,
};
use egui_extras::install_image_loaders;

mod gui;
mod main_menu;
mod pause_menu;
mod settings_menu;
mod shop_menu;
mod stats_display;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngameMenu {
    Stats,
    Shop,
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(IngameMenu::Stats)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                main_menu::main_menu_system.run_if(
                    in_state(GameState::MainMenu).and_then(in_state(SettingsState::Closed)),
                ),
            )
            .add_systems(
                Update,
                (pause_menu::pause_menu_system, darken_bg)
                    .run_if(in_state(PauseState::Paused).and_then(in_state(SettingsState::Closed))),
            )
            .add_systems(
                Update,
                (settings_menu::settings_menu_system, darken_bg)
                    .run_if(in_state(SettingsState::Open)),
            )
            .add_systems(Update, gui::gui_system.run_if(in_state(GameState::Game)))
            .add_systems(OnEnter(GameStage::Stopped), on_player_stop)
            .add_systems(
                Update,
                (
                    stats_display::stats_display_system.run_if(in_state(IngameMenu::Stats)),
                    shop_menu::shop_menu_system
                        .run_if(in_state(IngameMenu::Shop))
                        .after(stats_display::stats_display_system),
                )
                    .run_if(in_state(GameStage::Stopped)),
            );
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
        let shared = WidgetVisuals {
            bg_fill: Color32::default(),
            rounding: Rounding::ZERO,
            weak_bg_fill: Color32::default(),
            bg_stroke: Stroke::default(),
            fg_stroke: Stroke::default(),
            expansion: 0.,
        };
        style.visuals.widgets = Widgets {
            inactive: WidgetVisuals {
                bg_fill: Color32::from_black_alpha(200),
                weak_bg_fill: Color32::from_black_alpha(200),
                ..shared.clone()
            },
            hovered: WidgetVisuals {
                bg_fill: Color32::from_black_alpha(160),
                weak_bg_fill: Color32::from_black_alpha(160),
                ..shared.clone()
            },
            active: WidgetVisuals {
                bg_fill: Color32::from_black_alpha(230),
                weak_bg_fill: Color32::from_black_alpha(230),
                ..shared.clone()
            },
            ..default()
        }
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

fn on_player_stop(mut next_ingame_menu_state: ResMut<NextState<IngameMenu>>) {
    next_ingame_menu_state.set(IngameMenu::Stats);
}

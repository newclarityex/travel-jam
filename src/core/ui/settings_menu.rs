use crate::core::{
    audio::{MusicVolume, SFXVolume},
    SettingsState,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, RichText, Slider, TextureOptions},
    EguiContexts,
};
use bevy_kira_audio::AudioChannel;

pub fn settings_menu_system(
    mut contexts: EguiContexts,
    state: Res<State<SettingsState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
    mut music_volume: ResMut<MusicVolume>,
    mut sfx_volume: ResMut<SFXVolume>,
) {
    egui::Area::new("settings_menu")
        .anchor(Align2::LEFT_CENTER, [32., 0.])
        .order(egui::Order::Foreground)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(RichText::new("Settings").size(48.));
            ui.add_space(32.);

            ui.heading(RichText::new("Music Volume").size(24.));
            ui.add(Slider::new(&mut music_volume.0, 0.0..=2.0));
            ui.add_space(8.);

            ui.heading(RichText::new("SFX Volume").size(24.));
            ui.add(Slider::new(&mut sfx_volume.0, 0.0..=2.0));
            ui.add_space(32.);

            if ui
                .button(RichText::new("Back").size(32.))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                next_settings_state.set(SettingsState::Closed);
            }
        });
}

use bevy::{
    asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode, window::PrimaryWindow,
    window::WindowMode,
};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;

mod core;
mod debug;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    Game,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum PauseState {
    Paused,
    Running,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum SettingsState {
    Open,
    Closed,
}

fn main() {
    App::new()
        .insert_state(GameState::MainMenu)
        .insert_state(PauseState::Running)
        .insert_state(SettingsState::Closed)
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AudioPlugin,
            EguiPlugin,
            debug::DebugPlugin,
            core::CorePlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut camera_bundle = Camera2dBundle::default();

    // window height = 1600 world units
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(240.0);

    commands.spawn(camera_bundle);

    let mut window = window_query.get_single_mut().unwrap();
    window.mode = WindowMode::Fullscreen;
}

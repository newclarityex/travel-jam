use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod core;

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

fn main() {
    App::new()
        .insert_state(GameState::MainMenu)
        .insert_state(PauseState::Running)
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(core::CorePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    // window height = 1600 world units
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(240.0);

    println!("test camera");

    commands.spawn(camera_bundle);
}

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins(core::CorePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scale: 1.0,
    //         ..default()
    //     },
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());
}

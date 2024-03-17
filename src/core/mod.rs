use std::collections::HashMap;

use bevy::prelude::*;

use self::items::ItemsPlugin;

mod animations;
mod audio;
mod camera;
mod environment;
mod items;
mod pause_manager;
mod player;
mod ui;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    Game,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStage {
    Stopped,
    Sledding,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum SettingsState {
    Open,
    Closed,
}

#[derive(Clone)]
struct HitboxAsset {
    hitbox_handle: Handle<Image>,
    sprite_handle: Handle<Image>,
}

#[derive(Resource, Default)]
struct HitboxAssets(HashMap<String, HitboxAsset>);

#[derive(Component)]
struct CleanupEntity;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::MainMenu)
            .insert_state(SettingsState::Closed)
            .insert_state(GameStage::Sledding)
            .add_plugins((
                ui::UIPlugin,
                animations::AnimationPlugin,
                player::PlayerPlugin,
                audio::AudioPlugin,
                environment::EnvironmentPlugin,
                items::ItemsPlugin,
                pause_manager::PauseManagerPlugin,
                camera::CameraPlugin,
            ))
            .add_systems(OnExit(GameState::Game), cleanup_system);
    }
}

fn cleanup_system(mut commands: Commands, query: Query<Entity, With<CleanupEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

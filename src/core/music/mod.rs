use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::GameState;

pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), start_menu_music)
            .add_systems(OnExit(GameState::MainMenu), stop_menu_music);
    }
}

#[derive(Resource)]
struct MenuMusic(Handle<AudioInstance>);

fn start_menu_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let asset_handle = asset_server.load("audio/music/menu_music.mp3");
    let instance_handle = audio.play(asset_handle).looped().handle();
    commands.insert_resource(MenuMusic(instance_handle));
}

fn stop_menu_music(handle: Res<MenuMusic>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    let Some(instance) = audio_instances.get_mut(&handle.0) else {
        return;
    };

    instance.stop(AudioTween::default());
}

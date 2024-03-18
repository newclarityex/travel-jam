use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::core::GameState;

#[derive(Resource)]
pub struct MusicChannel;

#[derive(Resource)]
pub struct MusicVolume(pub f64);

#[derive(Resource)]
pub struct SFXChannel;

#[derive(Resource)]
pub struct SFXVolume(pub f64);

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<MusicChannel>()
            .insert_resource(MusicVolume(1.))
            .add_audio_channel::<SFXChannel>()
            .insert_resource(SFXVolume(1.))
            .add_systems(OnEnter(GameState::MainMenu), start_menu_music)
            .add_systems(OnExit(GameState::MainMenu), stop_menu_music)
            .add_systems(OnEnter(GameState::Game), start_game_music)
            .add_systems(OnExit(GameState::Game), stop_game_music)
            .add_systems(Update, update_volume);
    }
}

#[derive(Resource)]
struct MenuMusic(Handle<AudioInstance>);

#[derive(Resource)]
struct GameMusic(Handle<AudioInstance>);

fn start_menu_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    handle: Res<MenuMusic>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        instance.stop(AudioTween::default());
    };

    let asset_handle = asset_server.load("audio/music/menu.ogg");
    let instance_handle = music_channel.play(asset_handle).looped().handle();
    commands.insert_resource(MenuMusic(instance_handle));
}

fn stop_menu_music(handle: Res<MenuMusic>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    let Some(instance) = audio_instances.get_mut(&handle.0) else {
        return;
    };

    instance.stop(AudioTween::default());
}

fn start_game_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    prev_handle: Res<GameMusic>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        instance.stop(AudioTween::default());
    };

    instance.stop(AudioTween::default());
    let asset_handle = asset_server.load("audio/music/game.ogg");
    let instance_handle = music_channel.play(asset_handle).looped().handle();
    commands.insert_resource(GameMusic(instance_handle));
}

fn stop_game_music(handle: Res<GameMusic>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    let Some(instance) = audio_instances.get_mut(&handle.0) else {
        return;
    };

    instance.stop(AudioTween::default());
}

fn update_volume(
    music_channel: Res<AudioChannel<MusicChannel>>,
    music_volume: Res<MusicVolume>,
    sfx_channel: Res<AudioChannel<SFXChannel>>,
    sfx_volume: Res<SFXVolume>,
) {
    music_channel.set_volume(music_volume.0);
    sfx_channel.set_volume(sfx_volume.0);
}

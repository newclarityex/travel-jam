use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_parallax::{ParallaxCameraComponent, ParallaxPlugin};
use bevy_rapier2d::prelude::*;

mod core;
mod debug;
mod utils;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AudioPlugin,
            EguiPlugin,
            debug::DebugPlugin,
            core::CorePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            ParallaxPlugin,
        ))
        .add_systems(PreStartup, setup_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.y = 64.;

    // window height = 240 world units
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(240.0);
    // camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.0);

    commands.spawn((
        MainCamera,
        camera_bundle,
        ParallaxCameraComponent::default(),
    ));
}

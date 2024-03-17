#![allow(warnings)]
use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_parallax::ParallaxPlugin;
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
        .run();
}

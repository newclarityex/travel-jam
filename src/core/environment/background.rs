use bevy::prelude::*;
use bevy_parallax::{CreateParallaxEvent, LayerData, LayerRepeat, LayerSpeed, RepeatStrategy};

use crate::core::camera::MainCamera;

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_backgrounds);
    }
}

fn background_layer_default() -> LayerData {
    LayerData {
        tile_size: Vec2::new(576.0, 324.0),
        cols: 1,
        rows: 1,
        scale: Vec2::ONE,
        // position: Vec2::new(0., 96.),
        position: Vec2::new(0., -80.),
        ..Default::default()
    }
}

pub fn setup_backgrounds(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let Ok(camera) = camera_query.get_single() else {
        eprintln!("Missing Main Camera");
        return;
    };

    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(1.),
                path: "sprites/backgrounds/1.png".to_string(),
                z: -7.,
                ..background_layer_default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.95),
                path: "sprites/backgrounds/2.png".to_string(),
                z: -6.,
                position: Vec2::new(0., 128.),
                ..background_layer_default()
            },
            LayerData {
                repeat: LayerRepeat::Horizontal(RepeatStrategy::MirrorHorizontally),
                speed: LayerSpeed::Horizontal(0.85),
                path: "sprites/backgrounds/3.png".to_string(),
                z: -5.,
                ..background_layer_default()
            },
            LayerData {
                repeat: LayerRepeat::Horizontal(RepeatStrategy::MirrorHorizontally),
                speed: LayerSpeed::Horizontal(0.75),
                path: "sprites/backgrounds/4.png".to_string(),
                z: -4.,
                ..background_layer_default()
            },
            LayerData {
                repeat: LayerRepeat::Horizontal(RepeatStrategy::MirrorHorizontally),
                speed: LayerSpeed::Horizontal(0.5),
                path: "sprites/backgrounds/5.png".to_string(),
                z: -3.,
                ..background_layer_default()
            },
            LayerData {
                repeat: LayerRepeat::Horizontal(RepeatStrategy::MirrorHorizontally),
                speed: LayerSpeed::Horizontal(0.4),
                path: "sprites/backgrounds/6.png".to_string(),
                z: -2.,
                ..background_layer_default()
            },
        ],
        camera,
    });
}

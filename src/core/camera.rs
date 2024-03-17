use crate::core::GameState;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_parallax::{ParallaxCameraComponent, ParallaxMoveEvent};
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera)
            .add_systems(
                Update,
                main_menu_camera.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(Update, update_camera_position);
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub target_position: Vec2,
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.transform.translation.y = 64.;

    // window height = 240 world units
    // camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(240.0);
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(480.0);
    // camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.0);

    commands.spawn((
        MainCamera {
            target_position: Vec2::ZERO,
        },
        camera_bundle,
        ParallaxCameraComponent::default(),
    ));
}

fn update_camera_position(
    mut camera_query: Query<(Entity, &mut Transform, &MainCamera)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    time: Res<Time>,
) {
    let Ok((camera_entity, mut camera_transform, camera)) = camera_query.get_single_mut() else {
        eprintln!("Missing Camera!");
        return;
    };

    let distance_diff = camera.target_position - camera_transform.translation.xy();

    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: Vec2::ZERO.lerp(distance_diff, 20. * time.delta_seconds()),
        camera: camera_entity,
    });
}

fn main_menu_camera(time: Res<Time>, mut camera_query: Query<&mut MainCamera>) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        eprintln!("Missing Camera!");
        return;
    };

    camera.target_position.x += time.delta_seconds() * 12.;
}

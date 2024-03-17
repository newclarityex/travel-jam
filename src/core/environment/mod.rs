use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    ops::Range,
};

use crate::{
    core::{GameState, HitboxAsset},
    utils::{convex_decomposition, image_convex_decomposition},
};

use super::{pause_manager::PauseState, player::Player, CleanupEntity, GameStage, HitboxAssets};

mod background;
mod chunk_rendering;

#[derive(Component)]
pub struct Yarn;

#[derive(Component)]
pub struct Catnip;

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(chunk_rendering::CurrentChunks(HashSet::new()))
            .insert_resource(HitboxAssets::default())
            .add_plugins(background::BackgroundPlugin)
            .add_systems(Startup, load_assets)
            .add_systems(OnEnter(GameState::Game), setup_environment)
            .add_systems(
                Update,
                update_bobbing_sprites.run_if(in_state(PauseState::Running)),
            )
            .add_systems(
                OnEnter(GameStage::Sledding),
                (chunk_rendering::cleanup_chunks),
            )
            .add_systems(OnEnter(GameState::Game), (chunk_rendering::cleanup_chunks))
            .add_systems(
                PreUpdate,
                (chunk_rendering::update_chunks, update_floor_hitbox)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut hitbox_assets: ResMut<HitboxAssets>) {
    let hitbox_path = "sprites/yarn/yarn.png";
    let hitbox_handle: Handle<Image> = asset_server.load(hitbox_path);

    let sprite_path = "sprites/yarn/yarn.png";
    let sprite_handle: Handle<Image> = asset_server.load(sprite_path);

    hitbox_assets.0.insert(
        "yarn".to_string(),
        HitboxAsset {
            hitbox_handle,
            sprite_handle,
        },
    );

    let hitbox_path = "sprites/catnip/catnip.png";
    let hitbox_handle: Handle<Image> = asset_server.load(hitbox_path);

    let sprite_path = "sprites/catnip/catnip.png";
    let sprite_handle: Handle<Image> = asset_server.load(sprite_path);

    hitbox_assets.0.insert(
        "catnip".to_string(),
        HitboxAsset {
            hitbox_handle,
            sprite_handle,
        },
    );
}

#[derive(Component)]
struct SledHill;

const HILL_OFFSET: Vec2 = Vec2::new(-600., 199.);
const HILL_COORDS: &'static [Vec2] = &[
    // Bottom Right
    Vec2::new(600., -270.),
    // Bottom Left
    Vec2::new(-600., -270.),
    // Top Left
    Vec2::new(-600., 238.),
    // Ledge
    Vec2::new(-343., 238.),
    Vec2::new(-337., 237.),
    Vec2::new(-331., 235.),
    Vec2::new(-329., 234.),
    Vec2::new(-200., 107.),
    Vec2::new(-129., 48.),
    Vec2::new(-49., -9.),
    Vec2::new(109., -93.),
    Vec2::new(257., -150.),
    // Vec2::new(-49.5, -500.),
    Vec2::new(470., -208.),
    Vec2::new(600., -270.),
];

const FLOOR_HITBOX_OFFSET: f32 = -9.;
#[derive(Component)]
struct FloorBody;

fn setup_environment(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SledHill,
        Collider::polyline(HILL_COORDS.to_vec(), None),
        SpriteBundle {
            texture: asset_server.load("sprites/hill/hill.png"),
            transform: Transform::from_translation(HILL_OFFSET.extend(11.)),
            ..default()
        },
        RigidBody::Fixed,
        CleanupEntity,
    ));
    commands.spawn((
        FloorBody,
        RigidBody::Fixed,
        Collider::polyline(
            vec![
                Vec2::new(-1000., FLOOR_HITBOX_OFFSET),
                Vec2::new(1000., FLOOR_HITBOX_OFFSET),
            ],
            None,
        ),
        SpatialBundle::default(),
        CleanupEntity,
    ));
}

fn update_floor_hitbox(
    player_query: Query<&Transform, With<Player>>,
    mut floor_body_query: Query<&mut Transform, (With<FloorBody>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        eprintln!("Player not found!");
        return;
    };
    let Ok(mut floor_body_transform) = floor_body_query.get_single_mut() else {
        eprintln!("FloorBody!");
        return;
    };

    floor_body_transform.translation.x = player_transform.translation.x;
}

#[derive(Component)]
struct BobbingSprite {
    spawn_pos: Vec2,
    offset: f32,
    elapsed: f32,
}

const BOBBING_OFFSET: f32 = 10.;
fn update_bobbing_sprites(
    time: Res<Time>,
    mut bobbing_sprite_query: Query<(&mut BobbingSprite, &mut Transform)>,
) {
    for (mut bobbing_sprite, mut transform) in bobbing_sprite_query.iter_mut() {
        bobbing_sprite.elapsed += time.delta_seconds();
        transform.translation.y = bobbing_sprite.spawn_pos.y
            + BOBBING_OFFSET * (bobbing_sprite.elapsed * 2. + bobbing_sprite.offset).sin();
    }
}

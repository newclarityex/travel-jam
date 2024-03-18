use bevy::{prelude::*, sprite::Anchor};
use bevy_collider_gen::Edges;
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
mod hill_coords;

#[derive(Component)]
pub struct Yarn;

#[derive(Component)]
pub struct BigYarn;

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
                (chunk_rendering::cleanup_chunks.after(chunk_rendering::update_chunks)),
            )
            .add_systems(OnExit(GameState::Game), (chunk_rendering::cleanup_chunks))
            .add_systems(OnEnter(GameState::Game), add_instructions)
            .add_systems(
                PreUpdate,
                (chunk_rendering::update_chunks, update_floor_hitbox)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut hitbox_assets: ResMut<HitboxAssets>) {
    // let hitbox_path = "sprites/hill/hill_hitbox.png";
    // let hitbox_handle: Handle<Image> = asset_server.load(hitbox_path);

    // let sprite_path = "sprites/hill/hill.png";
    // let sprite_handle: Handle<Image> = asset_server.load(sprite_path);

    // hitbox_assets.0.insert(
    //     "hill".to_string(),
    //     HitboxAsset {
    //         hitbox_handle,
    //         sprite_handle,
    //     },
    // );

    let hitbox_path = "sprites/yarn/big_yarn.png";
    let hitbox_handle: Handle<Image> = asset_server.load(hitbox_path);

    let sprite_path = "sprites/yarn/big_yarn.png";
    let sprite_handle: Handle<Image> = asset_server.load(sprite_path);

    hitbox_assets.0.insert(
        "big_yarn".to_string(),
        HitboxAsset {
            hitbox_handle,
            sprite_handle,
        },
    );

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

const HILL_OFFSET: Vec2 = Vec2::new(-712., 166.);

const FLOOR_HITBOX_OFFSET: f32 = -9.;
#[derive(Component)]
struct FloorBody;

fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hitbox_assets: Res<HitboxAssets>,
    image_assets: Res<Assets<Image>>,
) {
    // let hill = hitbox_assets.0.get("hill").unwrap().clone();
    // let Some(sprite_image) = image_assets.get(hill.hitbox_handle) else {
    //     eprintln!("Failed to get sprite image from handler");
    //     return;
    // };
    // let edges = Edges::from(sprite_image);
    // let coords = edges.single_image_edge_translated();
    // let mut prev_coord: Option<Vec2> = None;
    // let mut coords: Vec<Vec2> = coords
    //     .into_iter()
    //     .filter(|coord| {
    //         if let Some(found_prev) = prev_coord {
    //             if coord.x != found_prev.x && coord.y != found_prev.y {
    //                 prev_coord = Some(*coord);
    //                 return true;
    //             }
    //             return false;
    //         } else {
    //             prev_coord = Some(*coord);
    //             return true;
    //         }
    //     })
    //     .collect();

    // coords.push(Vec2::new(-600., 238.));

    // println!("{:?}", coords);

    let hill_coords = commands.spawn((
        SledHill,
        Collider::polyline(hill_coords::HILL_COORDS.to_vec(), None),
        // Collider::polyline(coords, None),
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

#[derive(Component)]
struct Instructions;

fn add_instructions(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Instructions,
        SpriteBundle {
            texture: asset_server.load("sprites/inputs/sliding_instructions.png"),
            transform: Transform::from_xyz(-952., 535., 10.),
            ..default()
        },
        CleanupEntity,
    ));
    commands.spawn((
        Instructions,
        SpriteBundle {
            texture: asset_server.load("sprites/inputs/jump_instructions.png"),
            transform: Transform::from_xyz(-952., 510., 10.),
            ..default()
        },
        CleanupEntity,
    ));
    commands.spawn((
        Instructions,
        SpriteBundle {
            texture: asset_server.load("sprites/inputs/lean_instructions.png"),
            transform: Transform::from_xyz(-952., 485., 10.),
            ..default()
        },
        CleanupEntity,
    ));
}

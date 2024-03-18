use super::{BigYarn, BobbingSprite, Catnip, Yarn};
use crate::core::HitboxAssets;
use crate::core::{player::Player, CleanupEntity};
use bevy::prelude::*;
use bevy_collider_gen::rapier2d::single_convex_hull_collider_translated;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    ops::Range,
};

const CHUNK_SIZE: f32 = 256.;
const FLOOR_OFFSET: f32 = -8.;
// const FLOOR_OFFSET: f32 = -104.;

const RENDER_DISTANCE_Y: u16 = 2;
const RENDER_DISTANCE_X: u16 = 3;
const COLLECTABLES_Y_FLOOR: f32 = 16.;
const COLLECTABLES_X_FLOOR: f32 = 16.;

// Yarn bundles spawned per chunk
const YARN_SPAWN_RATE: i32 = 1;

// Chance for catnip to spawn per chunk
const BIG_YARN_SPAWN_CHANCE: f32 = 0.15;

// Chance for catnip to spawn per chunk
const CATNIP_SPAWN_CHANCE: f32 = 0.15;

#[derive(Resource, Debug)]
pub struct CurrentChunks(pub HashSet<IVec2>);

#[derive(Component)]
struct FloorChunk;

#[derive(Component)]
pub struct RenderCleanup {
    chunk: IVec2,
}

fn get_chunks_needed(
    current_chunk: &IVec2,
    render_distance_x: u16,
    render_distance_y: u16,
) -> Vec<IVec2> {
    let render_distance_x = i32::from(render_distance_x);
    let render_distance_y = i32::from(render_distance_y);
    let mut chunks_needed: Vec<IVec2> = Vec::new();
    for x in (current_chunk.x - render_distance_x)..=(current_chunk.x + render_distance_x) {
        for y in (current_chunk.y - render_distance_y)..=(current_chunk.y + render_distance_y) {
            chunks_needed.push(IVec2::new(x, y));
        }
    }

    chunks_needed
}

pub fn cleanup_chunks(
    mut commands: Commands,
    render_cleanup_query: Query<Entity, With<RenderCleanup>>,
    mut current_chunks: ResMut<CurrentChunks>,
) {
    current_chunks.0.clear();

    for entity in render_cleanup_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn try_spawn_catnip(
    mut commands: &mut Commands,
    image_assets: &Res<Assets<Image>>,
    hitbox_assets: &Res<HitboxAssets>,
    chunk: IVec2,
    x_range: Range<f32>,
    y_range: Range<f32>,
) {
    let mut rng = rand::thread_rng();

    let spawn_location = Vec2::new(
        rng.gen_range(x_range.clone()),
        rng.gen_range(y_range.clone()),
    );

    if spawn_location.x < COLLECTABLES_X_FLOOR || spawn_location.y < COLLECTABLES_Y_FLOOR {
        return;
    };

    let catnip = hitbox_assets.0.get("catnip").unwrap().clone();
    let Some(sprite_image) = image_assets.get(catnip.hitbox_handle) else {
        eprintln!("Failed to get sprite image from handler");
        return;
    };
    let Some(collider) = single_convex_hull_collider_translated(sprite_image) else {
        eprintln!("Failed to create catnip collider");
        return;
    };

    commands.spawn((
        Catnip,
        BobbingSprite {
            spawn_pos: spawn_location,
            offset: rng.gen_range((0.)..(PI * 2.)),
            elapsed: 0.,
        },
        SpriteBundle {
            texture: catnip.sprite_handle,
            transform: Transform::from_translation(spawn_location.extend(20.)),
            ..default()
        },
        collider,
        Sensor,
        RenderCleanup { chunk },
        CleanupEntity,
    ));
}

fn try_spawn_yarn(
    mut commands: &mut Commands,
    image_assets: &Res<Assets<Image>>,
    hitbox_assets: &Res<HitboxAssets>,
    chunk: IVec2,
    x_range: Range<f32>,
    y_range: Range<f32>,
) {
    let mut rng = rand::thread_rng();

    let spawn_location = Vec2::new(
        rng.gen_range(x_range.clone()),
        rng.gen_range(y_range.clone()),
    );

    if spawn_location.x < COLLECTABLES_X_FLOOR || spawn_location.y < COLLECTABLES_Y_FLOOR {
        return;
    };

    let yarn = hitbox_assets.0.get("big_yarn").unwrap().clone();
    let Some(sprite_image) = image_assets.get(yarn.hitbox_handle) else {
        eprintln!("Failed to get sprite image from handler");
        return;
    };
    let Some(collider) = single_convex_hull_collider_translated(sprite_image) else {
        eprintln!("Failed to create yarn collider");
        return;
    };

    commands.spawn((
        BigYarn,
        BobbingSprite {
            spawn_pos: spawn_location,
            offset: rng.gen_range((0.)..(PI * 2.)),
            elapsed: 0.,
        },
        SpriteBundle {
            texture: yarn.sprite_handle,
            transform: Transform::from_translation(spawn_location.extend(20.)),
            ..default()
        },
        collider,
        Sensor,
        RenderCleanup { chunk },
        CleanupEntity,
    ));
}

const FLOOR_CHUNKS: &'static [&'static str] = &[
    "sprites/floor/0.png",
    "sprites/floor/1.png",
    "sprites/floor/2.png",
];

const CLOUD_MIN_HEIGHT: f32 = 1_000.;
const CLOUD_DOUBLE_HEIGHT: f32 = 2_500.;

#[derive(Component)]
pub struct Cloud {
    speed: f32,
}

const CLOUD_SPRITES: &'static [&'static str] = &[
    "sprites/clouds/0.png",
    "sprites/clouds/1.png",
    "sprites/clouds/2.png",
];
// Chance for catnip to spawn per chunk
// const CLOUD_SPAWN_CHANCE: f32 = 0.15;
const CLOUD_SPAWN_CHANCE: f32 = 0.45;

fn try_spawn_cloud(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    chunk: IVec2,
    x_range: Range<f32>,
    y_range: Range<f32>,
) {
    let mut rng = rand::thread_rng();

    let spawn_location = Vec2::new(
        rng.gen_range(x_range.clone()),
        rng.gen_range(y_range.clone()),
    );

    if spawn_location.x < COLLECTABLES_X_FLOOR || spawn_location.y < CLOUD_MIN_HEIGHT {
        return;
    };

    commands.spawn((
        Cloud {
            speed: rng.gen_range((5.)..(20.)),
        },
        SpriteBundle {
            texture: asset_server.load(CLOUD_SPRITES.choose(&mut rng).unwrap().to_string()),
            transform: Transform::from_translation(spawn_location.extend(-0.5)),
            ..default()
        },
        RenderCleanup { chunk },
        CleanupEntity,
    ));
}

pub fn move_clouds(mut query: Query<(&Cloud, &mut Transform)>, time: Res<Time>) {
    for (cloud, mut transform) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * cloud.speed;
    }
}

pub fn update_chunks(
    mut commands: Commands,
    hitbox_assets: Res<HitboxAssets>,
    image_assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    render_cleanup_query: Query<(Entity, &RenderCleanup)>,
    mut current_chunks: ResMut<CurrentChunks>,
) {
    // Create new chunks
    let Ok(transform) = player_query.get_single() else {
        return;
    };

    let current_pos = transform.translation;
    let current_chunk = (current_pos.xy() / CHUNK_SIZE).as_ivec2();

    let chunks_needed = get_chunks_needed(&current_chunk, RENDER_DISTANCE_X, RENDER_DISTANCE_Y);

    let mut rng = rand::thread_rng();
    for chunk in &chunks_needed {
        if current_chunks.0.contains(&chunk) {
            continue;
        };

        current_chunks.0.insert(*chunk);

        let chunk_pos = chunk.as_vec2() * CHUNK_SIZE;
        let chunk_x_range = (chunk_pos.x)..(chunk_pos.x + CHUNK_SIZE);
        let chunk_y_range = (chunk_pos.y)..(chunk_pos.y + CHUNK_SIZE);

        for i in 0..YARN_SPAWN_RATE {
            let spawn_location = Vec2::new(
                rng.gen_range(chunk_x_range.clone()),
                rng.gen_range(chunk_y_range.clone()),
            );

            if spawn_location.x < COLLECTABLES_X_FLOOR || spawn_location.y < COLLECTABLES_Y_FLOOR {
                continue;
            };

            let init_angle = rng.gen_range((0.)..(2. * PI));

            for j in 0..3 {
                let mut spawn_location = spawn_location.clone();
                let angle = PI * 2. / 3. * (j as f32) + init_angle;
                let bundle_distance = 25.;
                spawn_location += Vec2::from_angle(angle) * bundle_distance;

                let yarn = hitbox_assets.0.get("yarn").unwrap().clone();
                let Some(sprite_image) = image_assets.get(yarn.hitbox_handle) else {
                    eprintln!("Failed to get sprite image from handler");
                    continue;
                };
                let Some(collider) = single_convex_hull_collider_translated(sprite_image) else {
                    eprintln!("Failed to create yarn collider");
                    continue;
                };

                commands.spawn((
                    Yarn,
                    BobbingSprite {
                        spawn_pos: spawn_location,
                        offset: rng.gen_range((0.)..(PI * 2.)),
                        elapsed: 0.,
                    },
                    SpriteBundle {
                        texture: yarn.sprite_handle,
                        transform: Transform::from_translation(spawn_location.extend(20.)),
                        ..default()
                    },
                    collider,
                    Sensor,
                    RenderCleanup { chunk: *chunk },
                    CleanupEntity,
                ));
            }
        }

        if rng.gen_bool(BIG_YARN_SPAWN_CHANCE.into()) {
            try_spawn_yarn(
                &mut commands,
                &image_assets,
                &hitbox_assets,
                chunk.clone(),
                chunk_x_range.clone(),
                chunk_y_range.clone(),
            );
        }

        if rng.gen_bool(CATNIP_SPAWN_CHANCE.into()) {
            try_spawn_catnip(
                &mut commands,
                &image_assets,
                &hitbox_assets,
                chunk.clone(),
                chunk_x_range.clone(),
                chunk_y_range.clone(),
            );
        }
        let mut cloud_chance = CLOUD_SPAWN_CHANCE.into();
        if chunk_pos.y >= CLOUD_DOUBLE_HEIGHT {
            cloud_chance *= 2.;
        }

        if rng.gen_bool(cloud_chance) {
            try_spawn_cloud(
                &mut commands,
                &asset_server,
                chunk.clone(),
                chunk_x_range.clone(),
                chunk_y_range.clone(),
            );
        }

        if chunk.y == 0 {
            // let mut current_floors = current_chunks
            //     .0
            //     .iter()
            //     .filter(|chunk| chunk.y == 0)
            //     .map(|chunk| chunk.x)
            //     .collect::<Vec<i32>>();
            // current_floors.sort();
            // println!("spawned {:?}, current: {:?}", chunk.x, current_floors);
            let floor_chunk = FLOOR_CHUNKS.choose(&mut rng);
            commands.spawn((
                FloorChunk,
                SpriteBundle {
                    texture: asset_server.load(floor_chunk.unwrap().to_string()),
                    transform: Transform::from_xyz(
                        chunk_pos.x,
                        chunk_pos.y * CHUNK_SIZE + FLOOR_OFFSET,
                        10.,
                    ),
                    ..default()
                },
                RenderCleanup { chunk: *chunk },
            ));
        }
    }

    // Clear unused chunks
    for (entity, info) in render_cleanup_query.iter() {
        let x_diff = (info.chunk.x - current_chunk.x).abs();
        let y_diff = (info.chunk.y - current_chunk.y).abs();
        if x_diff > (RENDER_DISTANCE_X).into() || y_diff > (RENDER_DISTANCE_Y).into() {
            current_chunks.0.remove(&info.chunk);
            commands.entity(entity).despawn_recursive();
        }
    }
}

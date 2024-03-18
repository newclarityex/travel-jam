use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

use crate::core::CleanupEntity;

use super::{
    animations::{AnimationData, AnimationsManager},
    pause_manager::PauseState,
    player::{Player, PlayerState},
    GameStage, GameState,
};
use bevy::{prelude::*, sprite::Anchor, utils::petgraph::matrix_graph::Zero};
use bevy_rapier2d::prelude::*;

#[derive(Eq, Hash, PartialEq)]
pub enum Item {
    SodaBooster,
    FireworkBooster,
    RocketBooster,
    SingleBalloon,
    TripleBalloons,
    HotAirBalloon,
    GliderBalloon,
    RacingVehicle,
    SkiingVehicle,
}

#[derive(Resource, Default)]
pub struct Inventory {
    pub yarn: i32,
    pub items: HashSet<Item>,
}

impl Inventory {
    pub fn buy_item(&mut self, item: Item, prices: &HashMap<Item, i32>) -> Result<(), ()> {
        let Some(item_price) = prices.get(&item) else {
            eprintln!("Tried to buy item that doesn't exist.");
            return Err(());
        };

        if *item_price > self.yarn {
            return Err(());
        }

        self.yarn -= item_price;
        self.items.insert(item);

        return Ok(());
    }
}

#[derive(Resource)]
pub struct ItemPrices(pub HashMap<Item, i32>);

impl ItemPrices {
    fn new() -> ItemPrices {
        let mut price_map = HashMap::new();
        price_map.insert(Item::SingleBalloon, 10);
        price_map.insert(Item::TripleBalloons, 50);
        price_map.insert(Item::HotAirBalloon, 200);
        price_map.insert(Item::GliderBalloon, 1000);
        price_map.insert(Item::SodaBooster, 10);
        price_map.insert(Item::FireworkBooster, 150);
        price_map.insert(Item::RocketBooster, 500);
        price_map.insert(Item::RacingVehicle, 10);
        price_map.insert(Item::SkiingVehicle, 50);
        return ItemPrices(price_map);
    }
}

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Inventory::default())
            .insert_resource(ItemPrices::new())
            .add_systems(Startup, setup_items)
            .add_systems(OnEnter(GameStage::Stopped), reset_booster)
            .add_systems(OnExit(GameState::Game), reset_booster)
            .add_systems(
                Update,
                (
                    apply_booster,
                    apply_balloons,
                    apply_vehicle,
                    update_balloon_rotation,
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GameStage::Sledding)),
            )
            .add_systems(
                Update,
                handle_boosting
                    .run_if(in_state(PlayerState::Sliding))
                    .run_if(in_state(GameStage::Sledding))
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

fn setup_items(mut inventory: ResMut<Inventory>) {
    *inventory = Inventory::default();
    // inventory.items.insert(Item::RocketBooster);
    // inventory.items.insert(Item::GliderBalloon);
    // inventory.items.insert(Item::SkiingVehicle);
}

#[derive(Component)]
pub struct Booster {
    max_fuel: f32,
    fuel: f32,
    force: f32,
    disconnected: bool,
}

#[derive(Component)]
struct BoosterEffect;

#[derive(Component)]
struct SodaBooster;

#[derive(Component)]
struct FireworkBooster;

#[derive(Component)]
struct RocketBooster;

fn apply_booster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: ResMut<Inventory>,
    current_booster_query: Query<
        (
            Entity,
            Has<SodaBooster>,
            Has<FireworkBooster>,
            Has<RocketBooster>,
        ),
        With<Booster>,
    >,
    mut player_query: Query<(&Player, Entity, &mut GravityScale)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok((player, player_entity, mut player_grav_scale)) = player_query.get_single_mut() else {
        eprintln!("Missing Player!");
        return;
    };

    let current_booster = current_booster_query.get_single();

    if inventory.items.contains(&Item::RocketBooster) {
        if let Ok((current_booster, _, _, rocket_booster)) = current_booster {
            if rocket_booster {
                return;
            };
            commands.entity(current_booster).despawn_recursive();
        }

        let texture = asset_server.load("sprites/items/rocket_booster.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(22.0, 7.0), 20, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let blasting_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 20,
            frame_durations: vec![6; 20],
            anchor: Anchor::Center,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("blasting", blasting_animation);
        animations_manager.play("blasting");
        animations_manager.time_scale = 0.;

        let booster = commands
            .spawn((
                RocketBooster,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(0., -2., 3.),
                    ..default()
                },
                Booster {
                    // max_fuel: 10000.,
                    // fuel: 10000.,
                    max_fuel: 15.,
                    fuel: 15.,
                    force: 2_500.,
                    disconnected: false,
                },
                animations_manager,
            ))
            .set_parent(player_entity)
            .id();

        let texture = asset_server.load("sprites/effects/rocket_fire.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(8.0, 9.0), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let blasting_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 6,
            frame_durations: vec![6; 20],
            anchor: Anchor::CenterRight,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("blasting", blasting_animation);
        animations_manager.play("blasting");
        animations_manager.looping = true;
        commands
            .spawn((
                BoosterEffect,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(-10., 0., 4.),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                animations_manager,
            ))
            .set_parent(booster);
    } else if inventory.items.contains(&Item::FireworkBooster) {
        if let Ok((current_booster, _, firework_booster, _)) = current_booster {
            if firework_booster {
                return;
            };
            commands.entity(current_booster).despawn_recursive();
        }

        let booster = commands
            .spawn((
                FireworkBooster,
                SpriteBundle {
                    texture: asset_server.load("sprites/items/firework_booster.png"),
                    transform: Transform::from_xyz(0., -2., 3.),
                    ..default()
                },
                Booster {
                    max_fuel: 5.,
                    fuel: 5.,
                    force: 2_000.,
                    disconnected: false,
                },
            ))
            .set_parent(player_entity)
            .id();

        let texture = asset_server.load("sprites/effects/firework_sparks.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(58.0, 28.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let blasting_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 4,
            frame_durations: vec![4; 20],
            anchor: Anchor::CenterRight,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("blasting", blasting_animation);
        animations_manager.play("blasting");
        animations_manager.looping = true;
        commands
            .spawn((
                BoosterEffect,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(-6., 0., 4.),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                animations_manager,
            ))
            .set_parent(booster);
    } else if inventory.items.contains(&Item::SodaBooster) {
        if let Ok((current_booster, soda_booster, _, _)) = current_booster {
            if soda_booster {
                return;
            };
            commands.entity(current_booster).despawn_recursive();
        }
        let texture = asset_server.load("sprites/items/soda_booster.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 7.0), 7, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let blasting_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 7,
            frame_durations: vec![6; 7],
            anchor: Anchor::Center,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("blasting", blasting_animation);
        animations_manager.play("blasting");
        animations_manager.time_scale = 0.;

        let booster = commands
            .spawn((
                SodaBooster,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(0., -2., 3.),
                    ..default()
                },
                Booster {
                    max_fuel: 1.,
                    fuel: 1.,
                    force: 2_500.,
                    disconnected: false,
                },
                animations_manager,
            ))
            .set_parent(player_entity)
            .id();

        let texture = asset_server.load("sprites/effects/soda_spray.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 13.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let blasting_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 4,
            frame_durations: vec![6; 4],
            anchor: Anchor::CenterRight,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("blasting", blasting_animation);
        animations_manager.play("blasting");
        animations_manager.looping = true;
        commands
            .spawn((
                BoosterEffect,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(-10., 0., 4.),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                animations_manager,
            ))
            .set_parent(booster);
    } else {
        if let Ok((current_booster, _, _, _)) = current_booster {
            commands.entity(current_booster).despawn_recursive();
        }
    }
}

fn handle_boosting(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut booster_query: Query<
        (
            Entity,
            &mut Booster,
            Option<&mut AnimationsManager>,
            &mut Transform,
        ),
        Without<Player>,
    >,
    mut booster_effect_query: Query<&mut Visibility, With<BoosterEffect>>,
    mut player_query: Query<(&Player, &mut ExternalForce, &Velocity, &Transform)>,
) {
    let Ok((player, mut player_force, player_velocity, player_transform)) =
        player_query.get_single_mut()
    else {
        eprintln!("Missing Player");
        return;
    };
    let Ok((booster_entity, mut booster, mut booster_animations, mut booster_transform)) =
        booster_query.get_single_mut()
    else {
        return;
    };

    let booster_effect = booster_effect_query.get_single_mut();

    // && player.collisions.len() == 0
    if keys.pressed(KeyCode::Space) && booster.fuel > 0. {
        if let Ok(mut booster_effect) = booster_effect {
            *booster_effect = Visibility::Visible;
        }

        booster.fuel -= time.delta_seconds();

        if let Some(mut booster_animations) = booster_animations {
            let frame_count = booster_animations.get_current().unwrap().frame_count;
            let ratio = booster.fuel / booster.max_fuel;
            if ratio <= 0. {
                booster_animations.index = frame_count - 1;
            } else {
                let frame = frame_count as f32 * (1. - ratio);
                booster_animations.index = frame as usize;
            }
        }

        let axis_angle = player_transform.rotation.to_axis_angle();
        let force_angle = axis_angle.0.z * axis_angle.1;
        player_force.force += Vec2::from_angle(force_angle) * booster.force;
    } else {
        if let Ok(mut booster_effect) = booster_effect {
            *booster_effect = Visibility::Hidden;
        }
    }

    if booster.fuel <= 0. && !booster.disconnected {
        booster.disconnected = true;

        let mut booster_commands = commands.entity(booster_entity);
        booster_commands.remove_parent();
        booster_commands.insert((
            GravityScale(1.),
            Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            Sensor,
            RigidBody::Dynamic,
            Collider::capsule(Vec2::new(-2., 0.), Vec2::new(-2., 0.), 0.5),
            ColliderMassProperties::Mass(2.),
            Velocity {
                linvel: player_velocity.linvel,
                angvel: player_velocity.angvel,
            },
        ));

        booster_transform.rotation = player_transform.rotation;
        booster_transform.translation.x += player_transform.translation.x;
        booster_transform.translation.y += player_transform.translation.y;
    }
}

fn reset_booster(
    mut commands: Commands,
    booster_query: Query<Entity, With<Booster>>,
    mut player_query: Query<(&mut ExternalForce)>,
) {
    let Ok(mut player_force) = player_query.get_single_mut() else {
        println!("No Player");
        return;
    };

    player_force.force = Vec2::ZERO;

    let Ok(booster_entity) = booster_query.get_single() else {
        println!("No booster");
        return;
    };

    commands.entity(booster_entity).despawn_recursive();
}

#[derive(Component)]
pub struct Balloon;

#[derive(Component)]
struct SingleBalloon;

#[derive(Component)]
struct TripleBalloons;

#[derive(Component)]
struct HotAirBalloon;

#[derive(Component)]
struct GliderBalloon;

fn apply_balloons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: ResMut<Inventory>,
    current_balloon_query: Query<
        (
            Entity,
            Has<SingleBalloon>,
            Has<TripleBalloons>,
            Has<HotAirBalloon>,
            Has<GliderBalloon>,
        ),
        With<Balloon>,
    >,
    mut player_query: Query<(&mut Player, Entity, &mut GravityScale)>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Ok((mut player, player_entity, mut player_grav_scale)) = player_query.get_single_mut()
    else {
        eprintln!("Missing Player!");
        return;
    };

    let current_balloon = current_balloon_query.get_single();

    if inventory.items.contains(&Item::GliderBalloon) {
        player_grav_scale.0 = player.default_grav;
        player.max_fall_vel = None;
        player.gliding_scale = Some(15.0);
        player.float_val = 0.01;

        if let Ok((current_balloon, _, _, _, glider_balloon)) = current_balloon {
            if glider_balloon {
                return;
            };
            commands.entity(current_balloon).despawn();
        }

        commands
            .spawn((
                GliderBalloon,
                SpriteBundle {
                    transform: Transform::from_xyz(-16.5, 23., 2.),
                    texture: asset_server.load("sprites/items/glider_balloon.png"),
                    ..default()
                },
                Balloon,
            ))
            .set_parent(player_entity);
    } else if inventory.items.contains(&Item::HotAirBalloon) {
        player_grav_scale.0 = player.default_grav * 0.6;
        player.max_fall_vel = Some(-75.);
        player.float_val = 0.25;

        if let Ok((current_balloon, _, _, hot_air_balloon, _)) = current_balloon {
            if hot_air_balloon {
                return;
            };
            commands.entity(current_balloon).despawn();
        }

        let texture = asset_server.load("sprites/items/hot_air_balloon.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 81.0), 6, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let idle_animation = AnimationData {
            texture,
            layout: texture_atlas_layout,
            frame_count: 6,
            frame_durations: vec![6; 20],
            anchor: Anchor::Center,
        };

        let mut animations_manager = AnimationsManager::new();
        animations_manager.load_animation("idle", idle_animation);
        animations_manager.play("idle");
        animations_manager.looping = true;

        commands
            .spawn((
                HotAirBalloon,
                SpriteSheetBundle {
                    transform: Transform::from_xyz(0., 48., 2.),
                    ..default()
                },
                animations_manager,
                Balloon,
            ))
            .set_parent(player_entity);
    } else if inventory.items.contains(&Item::TripleBalloons) {
        player_grav_scale.0 = player.default_grav * 0.75;
        player.max_fall_vel = Some(-125.);
        player.float_val = 0.1;

        if let Ok((current_balloon, _, triple_balloons, _, _)) = current_balloon {
            if triple_balloons {
                return;
            };
            commands.entity(current_balloon).despawn();
        }

        commands
            .spawn((
                TripleBalloons,
                SpriteBundle {
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.35, -0.5)),
                        ..default()
                    },
                    texture: asset_server.load("sprites/items/triple_balloons.png"),
                    ..default()
                },
                Balloon,
            ))
            .set_parent(player_entity);
    } else if inventory.items.contains(&Item::SingleBalloon) {
        player_grav_scale.0 = player.default_grav * 0.9;
        player.max_fall_vel = Some(-150.);
        player.float_val = 0.1;

        if let Ok((current_balloon, single_balloon, _, _, _)) = current_balloon {
            if single_balloon {
                return;
            };
            commands.entity(current_balloon).despawn();
        }

        commands
            .spawn((
                SingleBalloon,
                SpriteBundle {
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.5, -0.5)),
                        ..default()
                    },
                    texture: asset_server.load("sprites/items/single_balloon.png"),
                    ..default()
                },
                Balloon,
            ))
            .set_parent(player_entity);
    } else {
        if let Ok((current_balloon, _, _, _, _)) = current_balloon {
            commands.entity(current_balloon).despawn();
        }
        player_grav_scale.0 = player.default_grav;
        player.max_fall_vel = None;
        player.gliding_scale = None;
    }
}

fn update_balloon_rotation(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut current_balloon_query: Query<
        &mut Transform,
        (
            With<Balloon>,
            Without<HotAirBalloon>,
            Without<GliderBalloon>,
            Without<Player>,
        ),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut balloon_transform) = current_balloon_query.get_single_mut() else {
        return;
    };

    balloon_transform.rotation = player_transform.rotation.inverse() * -1.;
}

#[derive(Component)]
pub struct VehicleTextures {
    pub open_texture: Handle<Image>,
    pub closed_texture: Handle<Image>,
}

#[derive(Component)]
struct Vehicle;

#[derive(Component)]
struct BoxVehicle;

#[derive(Component)]
struct RacingVehicle;

#[derive(Component)]
struct SkiingVehicle;

fn apply_vehicle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: ResMut<Inventory>,
    current_vehicle_query: Query<
        (
            Entity,
            Has<BoxVehicle>,
            Has<RacingVehicle>,
            Has<SkiingVehicle>,
        ),
        With<Vehicle>,
    >,
    mut player_query: Query<(Entity, &mut Friction, &mut Damping), With<Player>>,
) {
    let Ok((player_entity, mut friction, mut damping)) = player_query.get_single_mut() else {
        eprintln!("Missing Player!");
        return;
    };

    let current_vehicle = current_vehicle_query.get_single();

    if inventory.items.contains(&Item::SkiingVehicle) {
        if let Ok((current_vehicle, _, _, skiing_vehicle)) = current_vehicle {
            if skiing_vehicle {
                return;
            };
            commands.entity(current_vehicle).despawn();
        }

        friction.coefficient = 0.05;
        damping.linear_damping = 0.05;

        let vehicle = commands
            .spawn((
                Vehicle,
                SkiingVehicle,
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                },
                VehicleTextures {
                    open_texture: asset_server.load("sprites/vehicles/skiing_box/open.png"),
                    closed_texture: asset_server.load("sprites/vehicles/skiing_box/closed.png"),
                },
            ))
            .set_parent(player_entity);

        let mut player_commands = commands.entity(player_entity);
        player_commands.remove::<Collider>();
        player_commands.insert(Collider::round_cuboid(7., 5., 0.05));
    } else if inventory.items.contains(&Item::RacingVehicle) {
        if let Ok((current_vehicle, _, racing_vehicle, _)) = current_vehicle {
            if racing_vehicle {
                return;
            };
            commands.entity(current_vehicle).despawn();
        }

        friction.coefficient = 0.10;
        damping.linear_damping = 0.10;

        let vehicle = commands
            .spawn((
                Vehicle,
                RacingVehicle,
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                },
                VehicleTextures {
                    open_texture: asset_server.load("sprites/vehicles/racing_box/open.png"),
                    closed_texture: asset_server.load("sprites/vehicles/racing_box/closed.png"),
                },
            ))
            .set_parent(player_entity);

        let mut player_commands = commands.entity(player_entity);
        player_commands.remove::<Collider>();
        player_commands.insert(Collider::round_cuboid(7., 5., 0.05));
    } else {
        if let Ok((current_vehicle, box_vehicle, _, _)) = current_vehicle {
            if box_vehicle {
                return;
            }
            commands.entity(current_vehicle).despawn();
        }

        friction.coefficient = 0.15;
        damping.linear_damping = 0.15;

        let vehicle = commands
            .spawn((
                Vehicle,
                BoxVehicle,
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                },
                VehicleTextures {
                    open_texture: asset_server.load("sprites/vehicles/box/open.png"),
                    closed_texture: asset_server.load("sprites/vehicles/box/closed.png"),
                },
            ))
            .set_parent(player_entity);

        let mut player_commands = commands.entity(player_entity);
        player_commands.remove::<Collider>();
        player_commands.insert(Collider::round_cuboid(7., 5., 0.05));
    }
}

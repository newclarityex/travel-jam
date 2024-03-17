use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use crate::core::{
    animations::{AnimationData, AnimationsManager},
    pause_manager::PauseState,
    GameState,
};

use super::{CleanupEntity, GameStage};

mod collisions;
mod movement;

#[derive(States, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PlayerState {
    Pushing,
    Sliding,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(PlayerState::Pushing)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnExit(GameState::Game), handle_exit)
            .add_systems(OnEnter(PauseState::Paused), on_pause)
            .add_systems(OnEnter(PauseState::Running), on_unpause)
            .add_systems(OnEnter(GameStage::Sledding), start_sledding)
            .add_systems(
                PreUpdate,
                movement::reset_force
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(PauseState::Running)),
            )
            .add_systems(
                Update,
                (
                    movement::handle_pushing.run_if(in_state(PlayerState::Pushing)),
                    movement::handle_sliding.run_if(in_state(PlayerState::Sliding)),
                    collisions::update_collisions,
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(PauseState::Running))
                    .run_if(in_state(GameStage::Sledding)),
            )
            .add_systems(
                (PostUpdate),
                (
                    movement::update_camera_position,
                    movement::update_hide_state,
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub default_grav: f32,
    pub max_fall_vel: Option<f32>,
    pub gliding_scale: Option<f32>,
    pub float_val: f32,
    push_force: f32,
    jump_vel: f32,
    lean_force: f32,
    pub collisions: HashSet<Entity>,
    pub current_yarn_collected: i32,
    pub current_catnip_collected: i32,
    pub score: i32,
}

#[derive(Component)]
struct PlayerSprite;

const STARTING_POSITION: Vec2 = Vec2::new(-952., 445.);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    next_player_state.set(PlayerState::Pushing);

    let mut player = commands
        .spawn((
            Player {
                current_yarn_collected: 0,
                current_catnip_collected: 0,
                score: 0,
                default_grav: 3.,
                max_fall_vel: None,
                gliding_scale: None,
                float_val: 0.,
                push_force: 500.,
                jump_vel: 250.,
                lean_force: 5.,
                collisions: HashSet::new(),
            },
            ExternalForce::default(),
            ExternalImpulse::default(),
            RigidBody::Dynamic,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::default(),
            Collider::cuboid(1.0, 1.0),
            AdditionalMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vect::new(0., -3.),
                mass: 5.,
                principal_inertia: 200.,
            }),
            ReadMassProperties::default(),
            Ccd::enabled(),
            Damping {
                // angular_damping: 5.0,
                angular_damping: 50.0,
                ..default()
            },
            Friction {
                combine_rule: CoefficientCombineRule::Min,
                ..default()
            },
            SpatialBundle {
                transform: Transform::from_translation(STARTING_POSITION.extend(2.)),
                ..default()
            },
            GravityScale(3.),
            CleanupEntity,
        ))
        .insert(Restitution {
            coefficient: -5.,
            combine_rule: CoefficientCombineRule::Min,
        })
        .id();

    let player_sprite = commands
        .spawn((
            PlayerSprite,
            SpriteSheetBundle {
                texture: asset_server.load("sprites/player/cat.png"),
                transform: Transform::from_xyz(0., 16., 1.),
                ..default()
            },
        ))
        .set_parent(player);
}

fn handle_exit(mut next_player_state: ResMut<NextState<PlayerState>>) {
    next_player_state.set(PlayerState::Pushing);
}

fn start_sledding(mut player_query: Query<(&mut Player, &mut Transform)>) {
    let Ok((mut player, mut transform)) = player_query.get_single_mut() else {
        eprintln!("Player missing!");
        return;
    };

    transform.translation.x = STARTING_POSITION.x;
    transform.translation.y = STARTING_POSITION.y;
    transform.rotation = Quat::default();
    player.current_yarn_collected = 0;
    player.current_catnip_collected = 0;
    player.score = 0;
}

fn on_pause(mut query: Query<&mut AnimationsManager, With<Player>>) {
    let Ok(mut animations_manager) = query.get_single_mut() else {
        return;
    };

    animations_manager.paused = true;
}
fn on_unpause(mut query: Query<&mut AnimationsManager, With<Player>>) {
    let Ok(mut animations_manager) = query.get_single_mut() else {
        return;
    };

    animations_manager.paused = false;
}

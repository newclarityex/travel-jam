use bevy::{prelude::*, sprite, sprite::Anchor};

use crate::{GameState, PauseState};

use crate::core::animations::{
    AnimationCompleteEvent, AnimationData, AnimationLoopEvent, AnimationsManager,
};

mod controls;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnEnter(PauseState::Paused), on_pause)
            .add_systems(OnEnter(PauseState::Running), on_unpause)
            .add_systems(
                Update,
                controls::handle_pause_input.run_if(in_state(GameState::Game)),
            )
            .add_systems(Update, on_animation_complete)
            .add_systems(Update, on_animation_looped)
            .add_systems(OnExit(GameState::Game), destroy);
    }
}
pub enum PlayerState {
    Idle,
    Walking,
}

pub enum PlayerAnimation {
    Idle,
    Walking,
}

#[derive(Component)]
pub struct Player {
    velocity: Vec2,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("./sprites/player/walking.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 20.0), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let walking_animation = AnimationData {
        texture,
        layout: texture_atlas_layout,
        frame_count: 8,
        frame_durations: vec![180; 8],
        anchor: Anchor::Center,
    };

    let mut animations_manager = AnimationsManager::new();
    animations_manager.load_animation("walking", walking_animation);
    animations_manager.looping = true;
    animations_manager.play("walking");

    commands.spawn((
        Player {
            velocity: Vec2::new(0., 0.),
        },
        SpriteSheetBundle::default(),
        animations_manager,
    ));
}

fn on_animation_complete(
    mut ev_complete: EventReader<AnimationCompleteEvent>,
    query: Query<&AnimationsManager, With<Player>>,
) {
    for ev in ev_complete.read() {
        if query.get(ev.entity).is_err() {
            return;
        }
        // println!("Animation {:?} finished!", ev.animation);
    }
}

fn on_animation_looped(
    mut ev_loop: EventReader<AnimationLoopEvent>,
    query: Query<&AnimationsManager, With<Player>>,
) {
    for ev in ev_loop.read() {
        if query.get(ev.entity).is_err() {
            return;
        }
        // println!("Animation {:?} looped!", ev.animation);
    }
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

fn destroy(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

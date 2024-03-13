use bevy::prelude::*;

mod animations;
mod audio;
mod player;
mod ui;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::UIPlugin,
            animations::AnimationPlugin,
            player::PlayerPlugin,
            audio::AudioPlugin,
        ));
    }
}

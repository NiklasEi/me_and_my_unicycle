use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

mod actions;
mod audio;
mod levels;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::levels::LevelsPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use loading::LoadingPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(LevelsPlugin)
            .add_plugin(MenuPlugin);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Prepare,
    InLevel,
}

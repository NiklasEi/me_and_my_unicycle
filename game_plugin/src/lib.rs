use crate::loading::TextureAssets;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use nalgebra::Isometry2;

pub struct GamePlugin;

mod actions;
mod loading;
mod player;

use crate::actions::ActionsPlugin;
use crate::player::PlayerPlugin;
use loading::LoadingPlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

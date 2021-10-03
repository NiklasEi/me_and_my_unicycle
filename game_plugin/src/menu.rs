use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_game.system()));
    }
}

fn start_game(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Prepare).unwrap();
}

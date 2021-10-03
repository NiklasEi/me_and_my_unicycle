use crate::player::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Point;

pub struct LevelsPlugin;

pub enum Level {
    Tutorial,
}

pub struct StartingPoint {
    wheel: Point<f32>,
    body: Point<f32>,
    head: Point<f32>,
}

pub struct ForLevel;

impl Level {
    fn get_starting_points(&self) -> StartingPoint {
        StartingPoint {
            wheel: [0., 0.5 * PATH_HEIGTH + WHEEL_RADIUS].into(),
            body: [
                0.,
                0.5 * PATH_HEIGTH + 2. * WHEEL_RADIUS + 0.5 * BODY_LENGTH + BODY_RADIUS,
            ]
            .into(),
            head: [
                0.,
                0.5 * PATH_HEIGTH
                    + 2. * WHEEL_RADIUS
                    + BODY_LENGTH
                    + 2. * BODY_RADIUS
                    + HEAD_RADIUS,
            ]
            .into(),
        }
    }
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Level::Tutorial)
            .add_system_set(
                SystemSet::on_update(GameState::Prepare).with_system(move_to_level.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InLevel).with_system(clear_level.system()),
            );
    }
}

fn move_to_level(mut state: ResMut<State<GameState>>) {
    state.set(GameState::InLevel).unwrap();
}

fn clear_level(mut commands: Commands, level_entites: Query<Entity, With<ForLevel>>) {
    for entity in level_entites.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

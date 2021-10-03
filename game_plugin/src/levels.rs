use crate::actions::Actions;
use crate::player::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            )
            .add_system_set(SystemSet::on_update(GameState::InLevel).with_system(restart.system()));
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

fn restart(
    actions: Res<Actions>,
    mut wheel_query: Query<
        (&mut RigidBodyVelocity, &mut RigidBodyPosition),
        (With<Wheel>, Without<Body>, Without<Head>),
    >,
    mut body_query: Query<
        (&mut RigidBodyVelocity, &mut RigidBodyPosition),
        (With<Body>, Without<Wheel>, Without<Head>),
    >,
    mut head_query: Query<
        (&mut RigidBodyVelocity, &mut RigidBodyPosition),
        (With<Head>, Without<Wheel>, Without<Body>),
    >,
    level: Res<Level>,
) {
    if actions.restart {
        let starting_points = level.get_starting_points();
        let (mut wheel_velocity, mut wheel_position) = wheel_query.single_mut().unwrap();
        *wheel_velocity = RigidBodyVelocity::default();
        wheel_position.position = Isometry::from(starting_points.wheel);
        wheel_position.next_position = Isometry::from(starting_points.wheel);

        let (mut body_velocity, mut body_position) = body_query.single_mut().unwrap();
        *body_velocity = RigidBodyVelocity::default();
        body_position.position = Isometry::from(starting_points.body);
        body_position.next_position = Isometry::from(starting_points.body);

        let (mut head_velocity, mut head_position) = head_query.single_mut().unwrap();
        *head_velocity = RigidBodyVelocity::default();
        head_position.position = Isometry::from(starting_points.head);
        head_position.next_position = Isometry::from(starting_points.head);
    }
}

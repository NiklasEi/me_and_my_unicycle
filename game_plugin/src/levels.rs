use crate::actions::Actions;
use crate::loading::FontAssets;
use crate::lost::{ButtonInteraction, ButtonMaterials};
use crate::nalgebra::Isometry2;
use crate::player::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::prelude::*;

pub struct LevelsPlugin;

#[derive(PartialEq)]
pub enum Level {
    Tutorial,
    First,
    Second,
}

pub struct StartingPoint {
    wheel: Point<f32>,
    body: Point<f32>,
    head: Point<f32>,
}

pub struct ForLevel;

impl Level {
    pub fn get_starting_points(&self) -> StartingPoint {
        match self {
            _ => StartingPoint {
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
            },
        }
    }

    pub fn finish_line(&self) -> f32 {
        match self {
            Level::Tutorial => 800. * 3.,
            Level::First => 800. * 3.,
            Level::Second => 800. * 3.,
        }
    }

    pub fn holes(&self) -> Vec<[f32; 2]> {
        match self {
            Level::Tutorial => vec![],
            Level::First => vec![[864., 1000.]],
            Level::Second => vec![[800., 1250.]],
        }
    }

    pub fn next(&self) -> Option<Level> {
        match self {
            Level::Tutorial => Some(Level::First),
            Level::First => Some(Level::Second),
            Level::Second => None,
        }
    }

    pub fn colliders(&self) -> Vec<ColliderBundle> {
        let mut colliders = vec![];
        match self {
            Level::Tutorial => {
                colliders.push(build_collider(
                    Isometry::from(Point2::from([800.0 / PHYSICS_SCALE, 2.])),
                    ColliderShape::cuboid(2., 1.),
                ));
            }
            Level::First => {
                colliders.push(build_collider(
                    Isometry::from(Point2::from([800.0 / PHYSICS_SCALE, 2.])),
                    ColliderShape::cuboid(2., 1.),
                ));
                colliders.push(build_collider(
                    Isometry::from(Point2::from([(800.0 / PHYSICS_SCALE) * 2.5, 2.])),
                    ColliderShape::cuboid(2., 1.),
                ));
            }
            Level::Second => {
                colliders.push(build_collider(
                    Isometry2::new(
                        [800.0 / PHYSICS_SCALE, 2.].into(),
                        std::f32::consts::FRAC_PI_4,
                    ),
                    ColliderShape::cuboid(4., 1.),
                ));
            }
        }

        colliders
    }
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Level::Tutorial)
            .add_system_set(
                SystemSet::on_update(GameState::Prepare).with_system(prepare_level.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PrepareLevel).with_system(build_parcours.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PrepareLevel).with_system(start_level.system()),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InLevel).with_system(clear_level.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InLevel)
                    .with_system(restart.system())
                    .with_system(cross_finish_line.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Finished).with_system(show_finished_button.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Finished).with_system(next_level.system()),
            );
    }
}

fn prepare_level(mut state: ResMut<State<GameState>>) {
    state.set(GameState::PrepareLevel).unwrap();
}

fn start_level(mut state: ResMut<State<GameState>>) {
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
        let (mut wheel_velocity, mut wheel_position) = wheel_query.single_mut().unwrap();
        let (mut body_velocity, mut body_position) = body_query.single_mut().unwrap();
        let (mut head_velocity, mut head_position) = head_query.single_mut().unwrap();
        reset_level(
            &level,
            (&mut wheel_velocity, &mut wheel_position),
            (&mut body_velocity, &mut body_position),
            (&mut head_velocity, &mut head_position),
        )
    }
}

fn cross_finish_line(
    mut body_query: Query<&Transform, (With<Body>, Without<Wheel>, Without<Head>)>,
    level: Res<Level>,
    mut state: ResMut<State<GameState>>,
) {
    let body_transform = body_query.single_mut().unwrap();

    if body_transform.translation.x > level.finish_line() {
        warn!("Done");
        state.push(GameState::Finished).unwrap();
    }
}

pub fn reset_level(
    level: &Level,
    (wheel_velocity, mut wheel_position): (&mut RigidBodyVelocity, &mut RigidBodyPosition),
    (body_velocity, mut body_position): (&mut RigidBodyVelocity, &mut RigidBodyPosition),
    (head_velocity, mut head_position): (&mut RigidBodyVelocity, &mut RigidBodyPosition),
) {
    let starting_points = level.get_starting_points();
    *wheel_velocity = RigidBodyVelocity::default();
    wheel_position.position = Isometry::from(starting_points.wheel);
    wheel_position.next_position = Isometry::from(starting_points.wheel);
    *body_velocity = RigidBodyVelocity::default();
    body_position.position = Isometry::from(starting_points.body);
    body_position.next_position = Isometry::from(starting_points.body);
    *head_velocity = RigidBodyVelocity::default();
    head_position.position = Isometry::from(starting_points.head);
    head_position.next_position = Isometry::from(starting_points.head);
}

fn next_level(
    mut commands: Commands,
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
    mut level: ResMut<Level>,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if let Some(next_level) = level.next() {
                    *level = next_level;
                    commands.entity(button).despawn();
                    commands.entity(text).despawn();
                    state.replace(GameState::PrepareLevel).unwrap();
                    let (mut wheel_velocity, mut wheel_position) =
                        wheel_query.single_mut().unwrap();
                    let (mut body_velocity, mut body_position) = body_query.single_mut().unwrap();
                    let (mut head_velocity, mut head_position) = head_query.single_mut().unwrap();
                    reset_level(
                        &level,
                        (&mut wheel_velocity, &mut wheel_position),
                        (&mut body_velocity, &mut body_position),
                        (&mut head_velocity, &mut head_position),
                    );
                } else {
                    warn!("No more levels :(");
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

struct FinishedButton;

fn show_finished_button(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
    level: Res<Level>,
) {
    let is_last_level = level.next().is_none();
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(FinishedButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: if is_last_level {
                            "Nice!".to_string()
                        } else {
                            "Next!".to_string()
                        },
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn build_parcours(mut commands: Commands, level: Res<Level>) {
    let mut colliders = level.colliders();
    for collider in colliders.drain(..) {
        commands
            .spawn_bundle(collider)
            .insert(ColliderDebugRender::default())
            .insert(ColliderPositionSync::Discrete)
            .insert(Platform)
            .insert(ForLevel);
    }
}

fn build_collider(isometry: Isometry2<f32>, shape: ColliderShape) -> ColliderBundle {
    ColliderBundle {
        shape,
        position: ColliderPosition(isometry),
        ..Default::default()
    }
}

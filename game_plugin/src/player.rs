use crate::actions::Actions;
use crate::audio::PlaySoundEffect;
use crate::levels::{ForLevel, Level};
use crate::loading::TextureAssets;
use crate::lost::LostSystem;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::prelude::*;
use nalgebra::Isometry2;

pub struct PlayerPlugin;

pub const WHEEL_RADIUS: f32 = 1.;
pub const HEAD_RADIUS: f32 = 0.5;
pub const BODY_RADIUS: f32 = 0.5;
pub const BODY_LENGTH: f32 = 1.;
pub const PATH_HEIGTH: f32 = 1.0;

pub const PHYSICS_SCALE: f32 = 32.0;

pub struct Wheel;
pub struct Head;
pub struct Body;
pub struct Camera;
pub struct Platform;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(JumpBlock::NotBlocked)
            .add_system_set(
                SystemSet::on_enter(GameState::Prepare)
                    .with_system(setup_rapier_and_camera.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::PrepareLevel)
                    .with_system(prepare_player_and_platforms.system())
                    .with_system(draw_background.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InLevel)
                    .before(LostSystem::Lost)
                    .with_system(paddle_wheel.system())
                    .with_system(move_head.system())
                    .with_system(move_camera.system())
                    .with_system(jump.system())
                    .with_system(landing.system()),
            );
    }
}

fn setup_rapier_and_camera(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = PHYSICS_SCALE;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 300.0, 0.0));
    commands.spawn_bundle(camera).insert(Camera);
}

pub fn prepare_player_and_platforms(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ground(&mut commands);
    let head_id = spawn_head(&mut commands, &textures, &mut materials);
    let body_id = spawn_body(&mut commands, &textures, &mut materials);
    let wheel_id = spawn_wheel(&mut commands, &textures, &mut materials);

    let mut wheel_body_joint = BallJoint::new(
        Vec2::new(0.0, 0.0).into(),
        Vec2::new(0.0, -0.5 * BODY_LENGTH - BODY_RADIUS - WHEEL_RADIUS - 0.1).into(),
    );
    wheel_body_joint.motor_model = SpringModel::Disabled;
    commands
        .spawn()
        .insert(JointBuilderComponent::new(
            wheel_body_joint,
            wheel_id,
            body_id,
        ))
        .insert(ForLevel);

    let mut body_head_joint = BallJoint::new(
        Vec2::new(0.0, 0.5 * BODY_LENGTH + BODY_RADIUS).into(),
        Vec2::new(0.0, -0.5 * HEAD_RADIUS).into(),
    );
    body_head_joint.motor_model = SpringModel::Disabled;
    commands
        .spawn()
        .insert(JointBuilderComponent::new(
            body_head_joint,
            body_id,
            head_id,
        ))
        .insert(ForLevel);
}

fn paddle_wheel(
    time: Res<Time>,
    actions: Res<Actions>,
    mut wheel_query: Query<&mut RigidBodyVelocity, With<Wheel>>,
) {
    if actions.paddling.is_none() || actions.restart {
        return;
    }
    let speed = 20.;
    let movement = actions.paddling.unwrap() * speed * time.delta_seconds();
    for mut wheel_velocity in wheel_query.iter_mut() {
        wheel_velocity.angvel = wheel_velocity.angvel - movement;
        // player_velocity.linvel.data.0[0][0] += movement.x;
    }
}

fn move_camera(
    head_query: Query<&Transform, (With<Head>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Head>)>,
) {
    for head_transform in head_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.x = head_transform.translation.x;
        }
    }
}

fn move_head(
    time: Res<Time>,
    actions: Res<Actions>,
    mut head_query: Query<&mut RigidBodyVelocity, With<Head>>,
) {
    if actions.head_balance.is_none() || actions.restart {
        return;
    }
    let speed = 20.;
    let movement = actions.head_balance.unwrap() * speed * time.delta_seconds();
    for mut head_velocity in head_query.iter_mut() {
        // head_velocity.angvel = clamp(head_velocity.angvel - movement, -5., 5.);
        head_velocity.linvel.data.0[0][0] += movement;
    }
}

fn spawn_ground(commands: &mut Commands) {
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid((400.0 / PHYSICS_SCALE) * 5., PATH_HEIGTH),
            position: ColliderPosition(Isometry::from(Point2::from([
                (800.0 / PHYSICS_SCALE) * 2.,
                0.,
            ]))),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(Platform)
        .insert(ForLevel);
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(300.0 / PHYSICS_SCALE, PATH_HEIGTH),
            position: ColliderPosition(Isometry2::new(
                [-(400.0 / PHYSICS_SCALE), 300.0 / PHYSICS_SCALE].into(),
                std::f32::consts::FRAC_PI_2,
            )),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(ForLevel);
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(300.0 / PHYSICS_SCALE, PATH_HEIGTH),
            position: ColliderPosition(Isometry2::new(
                [(800.0 / PHYSICS_SCALE) * 4.5, 300.0 / PHYSICS_SCALE].into(),
                std::f32::consts::FRAC_PI_2,
            )),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(ForLevel);
}

fn spawn_body(
    commands: &mut Commands,
    textures: &TextureAssets,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: [
                0.,
                0.5 * PATH_HEIGTH + 2. * WHEEL_RADIUS + 0.5 * BODY_LENGTH + BODY_RADIUS,
            ]
            .into(),
            forces: RigidBodyForces {
                gravity_scale: 0.3,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(
                [0., -0.5 * BODY_LENGTH].into(),
                [0., 0.5 * BODY_LENGTH].into(),
                BODY_RADIUS,
            ),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(textures.body.clone().into()),
            transform: Transform {
                scale: Vec3::new(0.125, 0.125, 0.125),
                ..Transform::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Body)
        .insert(ForLevel)
        .id()
}

fn spawn_head(
    commands: &mut Commands,
    textures: &TextureAssets,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: [
                0.,
                0.5 * PATH_HEIGTH
                    + 2. * WHEEL_RADIUS
                    + BODY_LENGTH
                    + 2. * BODY_RADIUS
                    + HEAD_RADIUS,
            ]
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(HEAD_RADIUS),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(textures.bevy.clone().into()),
            transform: Transform {
                scale: Vec3::new(0.125, 0.125, 0.125),
                ..Transform::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Head)
        .insert(ForLevel)
        .id()
}

#[derive(PartialEq)]
enum JumpBlock {
    Blocked,
    NotBlocked,
}

fn jump(
    actions: Res<Actions>,
    mut wheel_query: Query<
        (Entity, &mut RigidBodyVelocity, &Transform),
        (With<Wheel>, Without<Body>),
    >,
    mut jump_block: ResMut<JumpBlock>,
    mut body_query: Query<&Transform, (With<Body>, Without<Wheel>)>,
    platform_query: Query<Entity, (With<Platform>, Without<Wheel>, Without<Body>)>,
    mut sound_effects: EventWriter<PlaySoundEffect>,
    narrow_phase: Res<NarrowPhase>,
) {
    // give it a frame until allowing the next jump...
    if *jump_block == JumpBlock::Blocked {
        *jump_block = JumpBlock::NotBlocked;
        return;
    }
    if !actions.jump || actions.restart {
        return;
    }
    if let Ok((wheel, mut wheel_velocity, wheel_transform)) = wheel_query.single_mut() {
        for platform in platform_query.iter() {
            if let Some(contact_pair) = narrow_phase.contact_pair(wheel.handle(), platform.handle())
            {
                if contact_pair.has_any_active_contact {
                    warn!("JUMP!");
                    *jump_block = JumpBlock::Blocked;
                    let body_transform = body_query.single_mut().unwrap();
                    let jump_direction = Vec2::new(
                        body_transform.translation.x - wheel_transform.translation.x,
                        body_transform.translation.y - wheel_transform.translation.y,
                    );
                    jump_direction.normalize();
                    sound_effects.send(PlaySoundEffect::Jump);
                    wheel_velocity.linvel.data.0[0][0] += jump_direction.x * 0.15;
                    wheel_velocity.linvel.data.0[0][1] += jump_direction.y * 0.15;
                    return;
                }
            }
        }
    } else {
        warn!("Why is there more than one player?");
    }
}

fn landing(
    mut contact_event: EventReader<ContactEvent>,
    mut sound_effects: EventWriter<PlaySoundEffect>,
) {
    for event in contact_event.iter() {
        if let ContactEvent::Started(_, _) = event {
            sound_effects.send(PlaySoundEffect::Land);
        }
    }
}

fn spawn_wheel(
    commands: &mut Commands,
    textures: &TextureAssets,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: [0., 0.5 * PATH_HEIGTH + WHEEL_RADIUS].into(),
            damping: RigidBodyDamping {
                angular_damping: 0.2.into(),
                ..RigidBodyDamping::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(WHEEL_RADIUS),
            flags: ColliderFlags::from(ActiveEvents::CONTACT_EVENTS),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(textures.bevy.clone().into()),
            transform: Transform {
                scale: Vec3::new(0.25, 0.25, 0.25),
                ..Transform::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Wheel)
        .insert(ForLevel)
        .id()
}

fn draw_background(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
) {
    let material = materials.add(textures.background_1.clone().into());
    for slot in 0..5 {
        commands
            .spawn_bundle(SpriteBundle {
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(slot as f32 * 800.0, 300.0, 0.0)),
                ..Default::default()
            })
            .insert(ForLevel);
    }
    if *level == Level::Tutorial {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(textures.tutorial.clone().into()),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-180.0, 250.0, 0.0));
                    transform.scale = Vec3::new(0.5, 0.5, 0.5);

                    transform
                },
                ..Default::default()
            })
            .insert(ForLevel);
    }
}

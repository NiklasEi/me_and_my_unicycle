use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_graphics.system())
                .with_system(setup_physics.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()));
    }
}

fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 32.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));

    commands.spawn_bundle(camera);
}

pub fn setup_physics(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground_size = 250.0;
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    let body = RigidBodyBundle {
        position: [0., 5.].into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::capsule([0., -0.5].into(), [0., 0.5].into(), 0.5),
        ..Default::default()
    };
    let mut sprite_transform = Transform::default();
    sprite_transform.scale = Vec3::new(0.125, 0.125, 0.125);
    let body_id = commands
        .spawn_bundle(body)
        .insert_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(textures.body.clone().into()),
            transform: sprite_transform,
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .id();

    let rad = 1.;

    let body = RigidBodyBundle {
        position: [0., rad].into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(rad),
        ..Default::default()
    };
    let mut sprite_transform = Transform::default();
    sprite_transform.scale = Vec3::new(0.25, 0.25, 0.25);
    let player_id = commands
        .spawn_bundle(body)
        .insert_bundle(collider)
        .insert_bundle(SpriteBundle {
            material: materials.add(textures.bevy.clone().into()),
            transform: sprite_transform,
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Player)
        .id();

    let joint = BallJoint::new(Vec2::new(0.0, 0.0).into(), Vec2::new(0.0, -2.05).into());
    commands
        .spawn()
        .insert(JointBuilderComponent::new(joint, player_id, body_id));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut RigidBodyVelocity, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 10.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_velocity in player_query.iter_mut() {
        player_velocity.angvel -= movement.x;
        // player_velocity.linvel.data.0[0][0] += movement.x;
    }
}

use crate::audio::PlaySoundEffect;
use crate::levels::{reset_level, Level};
use crate::loading::FontAssets;
use crate::player::*;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LostPlugin;

impl Plugin for LostPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_update(GameState::InLevel)
                    .with_system(lost.system().label(LostSystem::Lost)),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Lost).with_system(show_restart_button.system()),
            )
            .add_system_set(SystemSet::on_update(GameState::Lost).with_system(restart.system()));
    }
}

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
pub enum LostSystem {
    Lost,
}

fn lost(
    mut head_query: Query<Entity, (With<Head>, Without<Platform>)>,
    platform_query: Query<Entity, (With<Platform>, Without<Head>)>,
    narrow_phase: Res<NarrowPhase>,
    mut sounds: EventWriter<PlaySoundEffect>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok(head) = head_query.single_mut() {
        for platform in platform_query.iter() {
            if let Some(contact_pair) = narrow_phase.contact_pair(head.handle(), platform.handle())
            {
                if contact_pair.has_any_active_contact {
                    warn!("Lost!");
                    sounds.send(PlaySoundEffect::Loose);
                    state.push(GameState::Lost).unwrap();
                    return;
                }
            }
        }
    } else {
        warn!("Why is there more than one player?");
    }
}

fn restart(
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
    level: Res<Level>,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
    text_query: Query<Entity, With<Text>>,
) {
    for (button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn();
                commands.entity(text).despawn();
                state.replace(GameState::PrepareLevel).unwrap();
                let (mut wheel_velocity, mut wheel_position) = wheel_query.single_mut().unwrap();
                let (mut body_velocity, mut body_position) = body_query.single_mut().unwrap();
                let (mut head_velocity, mut head_position) = head_query.single_mut().unwrap();
                reset_level(
                    &level,
                    (&mut wheel_velocity, &mut wheel_position),
                    (&mut body_velocity, &mut body_position),
                    (&mut head_velocity, &mut head_position),
                );
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

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct RestartButton;

fn show_restart_button(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
) {
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
        .insert(RestartButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Again!".to_string(),
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

pub type ButtonInteraction<'a> = (
    Entity,
    &'a Interaction,
    &'a mut Handle<ColorMaterial>,
    &'a Children,
);

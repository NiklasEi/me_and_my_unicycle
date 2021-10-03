use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::InLevel).with_system(set_movement_actions.system()),
        );
    }
}

#[derive(Default, Debug)]
pub struct Actions {
    pub jump: bool,
    pub paddling: Option<f32>,
    pub head_balance: Option<f32>,
    pub restart: bool,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::PaddleBackward.just_released(&keyboard_input)
        || GameControl::PaddleBackward.pressed(&keyboard_input)
        || GameControl::PaddleForward.just_released(&keyboard_input)
        || GameControl::PaddleForward.pressed(&keyboard_input)
    {
        let mut paddling = actions.paddling.unwrap_or(0.);
        if GameControl::PaddleForward.just_released(&keyboard_input)
            || GameControl::PaddleBackward.just_released(&keyboard_input)
        {
            if GameControl::PaddleForward.pressed(&keyboard_input) {
                paddling = 1.;
            } else if GameControl::PaddleBackward.pressed(&keyboard_input) {
                paddling = -1.;
            } else {
                paddling = 0.;
            }
        } else if GameControl::PaddleForward.just_pressed(&keyboard_input) {
            paddling = 1.;
        } else if GameControl::PaddleBackward.just_pressed(&keyboard_input) {
            paddling = -1.;
        }
        actions.paddling = Some(paddling);
    } else {
        actions.paddling = None;
    }

    if GameControl::BalanceForward.just_released(&keyboard_input)
        || GameControl::BalanceForward.pressed(&keyboard_input)
        || GameControl::BalanceBackward.just_released(&keyboard_input)
        || GameControl::BalanceBackward.pressed(&keyboard_input)
    {
        let mut head_balance = actions.head_balance.unwrap_or(0.);
        if GameControl::BalanceForward.just_released(&keyboard_input)
            || GameControl::BalanceBackward.just_released(&keyboard_input)
        {
            if GameControl::BalanceForward.pressed(&keyboard_input) {
                head_balance = 1.;
            } else if GameControl::BalanceBackward.pressed(&keyboard_input) {
                head_balance = -1.;
            } else {
                head_balance = 0.;
            }
        } else if GameControl::BalanceForward.just_pressed(&keyboard_input) {
            head_balance = 1.;
        } else if GameControl::BalanceBackward.just_pressed(&keyboard_input) {
            head_balance = -1.;
        }
        actions.head_balance = Some(head_balance);
    } else {
        actions.head_balance = None;
    }

    actions.jump = GameControl::Jump.just_pressed(&keyboard_input);
    actions.restart = GameControl::Restart.just_pressed(&keyboard_input);
}

enum GameControl {
    BalanceForward,
    BalanceBackward,
    PaddleBackward,
    PaddleForward,
    Restart,
    Jump,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::BalanceForward => keyboard_input.just_released(KeyCode::Right),
            GameControl::BalanceBackward => keyboard_input.just_released(KeyCode::Left),
            GameControl::PaddleBackward => keyboard_input.just_released(KeyCode::A),
            GameControl::PaddleForward => keyboard_input.just_released(KeyCode::D),
            GameControl::Jump => keyboard_input.just_released(KeyCode::Space),
            GameControl::Restart => keyboard_input.just_released(KeyCode::R),
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::BalanceForward => keyboard_input.pressed(KeyCode::Right),
            GameControl::BalanceBackward => keyboard_input.pressed(KeyCode::Left),
            GameControl::PaddleBackward => keyboard_input.pressed(KeyCode::A),
            GameControl::PaddleForward => keyboard_input.pressed(KeyCode::D),
            GameControl::Jump => keyboard_input.pressed(KeyCode::Space),
            GameControl::Restart => keyboard_input.pressed(KeyCode::R),
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::BalanceForward => keyboard_input.just_pressed(KeyCode::Right),
            GameControl::BalanceBackward => keyboard_input.just_pressed(KeyCode::Left),
            GameControl::PaddleBackward => keyboard_input.just_pressed(KeyCode::A),
            GameControl::PaddleForward => keyboard_input.just_pressed(KeyCode::D),
            GameControl::Jump => keyboard_input.just_pressed(KeyCode::Space),
            GameControl::Restart => keyboard_input.just_pressed(KeyCode::R),
        }
    }
}

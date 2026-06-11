use std::ops::Deref;

use bevy::{
    app::{Plugin, Startup},
    ecs::{
        component::Component,
        message::MessageReader,
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    input::gamepad::GamepadButton::Start,
    platform::collections::Equivalent,
};

use crate::{key_input::KeyPressMessage, state::GameState, terminal::BufferSize};

pub struct TermshotGameplayPlugin;

impl Plugin for TermshotGameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        // app.add_systems(Startup, spawn_random_components);
        // app.insert_resource(State(Vec::new()));
    }
}

#[derive(Resource)]
pub struct Cursor {
    position: Position,
    state: DirectionState,
    history: Vec<(Position, DirectionState)>,
}

#[derive(Default)]
pub enum DirectionState {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Component)]
pub struct Position {
    x: u16,
    y: u16,
}

pub struct Character {
    char: char,
}

// #[derive(Resource)]
// pub enum TeleportState {
//     NotTeleporting,
//     IsTeleporting,
//     TeleportEnding,
// }

// #[derive(Resource)]
// pub enum JumpState {
//
// }

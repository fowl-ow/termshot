use std::ops::Deref;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        message::MessageReader,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::gamepad::GamepadButton::Start,
    platform::collections::Equivalent,
};

use crate::{key_input::CursorIntentMessage, state::GameState, terminal::BufferSize};

pub struct TermshotGameplayPlugin;

impl Plugin for TermshotGameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, cursor_intent_processor);
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

impl Cursor {
    fn increment(&mut self) {
        match self.state {
            DirectionState::Up => self.position.y += 1,
            DirectionState::Down => self.position.y -= 1,
            DirectionState::Left => self.position.x -= 1,
            DirectionState::Right => self.position.x += 1,
        }
    }
}

#[derive(Default)]
pub enum DirectionState {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Component, Clone, Copy)]
pub struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
pub struct Character {
    char: char,
}

fn cursor_intent_processor(
    mut cursor: ResMut<Cursor>,
    mut intent_reader: MessageReader<CursorIntentMessage>,
    // mut query: Query<(&Character, &Position)>,
    mut commands: Commands,
) {
    for intent in intent_reader.read() {
        match intent {
            CursorIntentMessage::GoBack => {}
            CursorIntentMessage::Print(key_code) => {
                if let Some(c) = key_code.as_char() {
                    spawn_character(c, cursor.position, &mut commands);
                    cursor.increment();
                }
            }
        }
    }
}

fn spawn_character(char: char, position: Position, commands: &mut Commands) {}

// fn match_state(history: Vec<Position, DirectionState>) -> Option<DirectionState> {
//
//     None
// }

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

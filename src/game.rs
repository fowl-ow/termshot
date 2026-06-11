use std::{fmt::Display, ops::Deref};

use bevy::{
    app::{Plugin, Startup, Update},
    asset::io::memory::Dir,
    ecs::{
        component::Component,
        message::MessageReader,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::gamepad::GamepadButton::Start,
    platform::collections::Equivalent,
};

use crate::{cursor, key_input::CursorIntentMessage, state::GameState, terminal::BufferSize};

pub struct TermshotGameplayPlugin;

impl Plugin for TermshotGameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Cursor::new());
        app.add_systems(Startup, move_cursor_to_center);
        app.add_systems(Update, cursor_intent_processor);
        // app.add_systems(Startup, spawn_random_components);
        // app.insert_resource(State(Vec::new()));
    }
}

fn move_cursor_to_center(buff_size: Res<BufferSize>, mut cursor: ResMut<Cursor>) {
    let center_x = buff_size.cols / 2;
    let center_y = buff_size.rows / 2;
    cursor.position.x = center_x;
    cursor.position.y = center_y;
}

#[derive(Resource)]
pub struct Cursor {
    pub position: Position,
    direction: Direction,
    history: Vec<(Character, Position, Direction)>,
}

impl Cursor {
    fn new() -> Self {
        Self {
            position: Position::new(0, 0),
            direction: Direction::default(),
            history: Vec::new(),
        }
    }

    fn increment(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn decrement(&mut self) {
        match self.direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x += 1,
            Direction::Right => self.position.x -= 1,
        }
        if let Some((_, _, dir)) = self.history.last()
            && *dir != self.direction
        {
            self.direction.transition(*dir);
        }
    }

    fn push_to_history(&mut self, char: Character) {
        self.history.push((char, self.position, self.direction));
    }

    fn ends_with(&self, target: &str) -> bool {
        self.history
            .iter()
            .rev()
            .map(|(c, _, _)| c.char)
            .take(target.chars().count())
            .eq(target.chars().rev())
    }

    fn find_direction_match(&self) -> Option<Direction> {
        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.ends_with(dir.as_str()) {
                return Some(*dir);
            }
        }
        None
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    fn transition(&mut self, desired: Self) {
        if *self != desired && !self.is_opposite(desired) {
            *self = desired;
        }
    }

    fn is_opposite(&self, other: Self) -> bool {
        self.opposite() == other
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Character {
    pub char: char,
}

impl Character {
    fn new(char: char) -> Self {
        Self { char }
    }
}

fn cursor_intent_processor(
    mut cursor: ResMut<Cursor>,
    mut intent_reader: MessageReader<CursorIntentMessage>,
    // mut query: Query<(&Character, &Position)>,
    mut commands: Commands,
) {
    for intent in intent_reader.read() {
        match intent {
            CursorIntentMessage::GoBack => {
                cursor.decrement();
            }
            CursorIntentMessage::Print(key_code) => {
                if let Some(c) = key_code.as_char() {
                    spawn_character(c, cursor.position, &mut commands);
                    cursor.push_to_history(Character::new(c));
                    if let Some(dir) = cursor.find_direction_match() {
                        cursor.direction.transition(dir);
                    }
                    cursor.increment();
                }
            }
        }
    }
}

fn spawn_character(char: char, position: Position, commands: &mut Commands) {
    commands.spawn((Character::new(char), position));
}

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

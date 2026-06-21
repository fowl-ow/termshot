use std::task::Poll::Pending;
use std::{fmt::Display, ops::Deref};

use bevy::prelude::*;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::io::memory::Dir,
    ecs::{
        component::Component,
        message::MessageReader,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::gamepad::GamepadButton::Start,
    platform::collections::Equivalent,
};

use crate::components::{Cursor, Position};
use crate::{map::SpatialGrid, state::GameState, terminal::BufferSize};

pub(super) fn plugin(app: &mut App) {
    // spawn cursor in poststartup so we can later first spawn other stuff in the map like walls and
    // then spawn the cursor where there is free space
    app.add_systems(PostStartup, spawn_cursor);
}

fn spawn_cursor(mut commands: Commands, buff_size: Res<BufferSize>) {
    let x = f32::from(buff_size.cols / 2);
    let y = f32::from(buff_size.rows / 2);

    commands.spawn((Cursor, Position { x, y }));
}

// ---------------------
// ---------------------
// ---------------------

// pub struct TermshotGameplayPlugin;
//
// impl Plugin for TermshotGameplayPlugin {
//     fn build(&self, app: &mut bevy::app::App) {
//         app.insert_resource(Cursor::new());
//         app.add_systems(Startup, move_cursor_to_center);
//         app.add_systems(Update, cursor_intent_processor);
//         // app.add_systems(Startup, spawn_random_components);
//         // app.insert_resource(State(Vec::new()));
//     }
// }
//
// fn move_cursor_to_center(buff_size: Res<BufferSize>, mut cursor: ResMut<Cursor>) {
//     let center_x = buff_size.cols / 2;
//     let center_y = buff_size.rows / 2;
//     cursor.position.x = center_x;
//     cursor.position.y = center_y;
// }
//
// #[derive(Resource)]
// pub struct Cursor {
//     pub position: Position,
//     pub direction: Direction,
//     pub history: Vec<Character>,
// }
//
// impl Cursor {
//     fn new() -> Self {
//         Self {
//             position: Position::new(0, 0),
//             direction: Direction::default(),
//             history: Vec::new(),
//         }
//     }
//
//     fn move_forward(&mut self) {
//         match self.direction {
//             Direction::Up => self.position.y -= 1,
//             Direction::Down => self.position.y += 1,
//             Direction::Left => self.position.x -= 1,
//             Direction::Right => self.position.x += 1,
//         }
//     }
//
//     fn get_rewind_pos(&self) -> Position {
//         self.position
//             .add_direction_and_get(self.direction.opposite())
//     }
//
//     fn rewind(&mut self, new_direction: Direction) {
//         self.position = self.get_rewind_pos();
//         self.direction = new_direction;
//     }
//
//     fn ends_with(&self, target: &str) -> bool {
//         self.history
//             .iter()
//             .rev()
//             .map(|c| c.char)
//             .take(target.chars().count())
//             .eq(target.chars().rev())
//     }
//
//     fn find_direction_match(&self) -> Option<Direction> {
//         for dir in &[
//             Direction::Up,
//             Direction::Down,
//             Direction::Left,
//             Direction::Right,
//         ] {
//             if self.ends_with(dir.as_str()) {
//                 return Some(*dir);
//             }
//         }
//         None
//     }
// }
//
// #[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Component)]
// pub enum Direction {
//     Up,
//     Down,
//     Left,
//     #[default]
//     Right,
// }
//
// impl Direction {
//     fn transition(&mut self, desired: Self) {
//         if *self != desired && !self.is_opposite(desired) {
//             *self = desired;
//         }
//     }
//
//     fn is_opposite(&self, other: Self) -> bool {
//         self.opposite() == other
//     }
//
//     fn opposite(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Down,
//             Direction::Down => Direction::Up,
//             Direction::Left => Direction::Right,
//             Direction::Right => Direction::Left,
//         }
//     }
//
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Direction::Up => "up",
//             Direction::Down => "down",
//             Direction::Left => "left",
//             Direction::Right => "right",
//         }
//     }
// }
//
// #[derive(Debug, Component, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Position {
//     pub x: u16,
//     pub y: u16,
// }
//
// #[derive(Component)]
// pub struct PendingDespawn;
//
// impl Position {
//     fn new(x: u16, y: u16) -> Self {
//         Self { x, y }
//     }
//
//     fn add_direction_and_get(&self, direction: Direction) -> Self {
//         match direction {
//             Direction::Up => Self {
//                 x: self.x,
//                 y: self.y - 1,
//             },
//             Direction::Down => Self {
//                 x: self.x,
//                 y: self.y + 1,
//             },
//             Direction::Left => Self {
//                 x: self.x - 1,
//                 y: self.y,
//             },
//             Direction::Right => Self {
//                 x: self.x + 1,
//                 y: self.y,
//             },
//         }
//     }
//
//     fn add_direction_and_mut(&mut self, direction: Direction) {
//         match direction {
//             Direction::Up => self.y -= 1,
//             Direction::Down => self.y += 1,
//             Direction::Left => self.x -= 1,
//             Direction::Right => self.x += 1,
//         };
//     }
// }
//
// #[derive(Component, Clone, Copy, PartialEq, PartialOrd, Eq)]
// pub struct Character {
//     pub char: char,
// }
//
// impl Character {
//     fn new(char: char) -> Self {
//         Self { char }
//     }
// }
//
// fn cursor_intent_processor(
//     mut cursor: ResMut<Cursor>,
//     mut grid: ResMut<SpatialGrid>,
//     mut intent_reader: MessageReader<CursorIntentMessage>,
//     mut query: Query<(Entity, &Direction, &Position), (With<Character>, With<Position>)>,
//     mut commands: Commands,
// ) {
//     for intent in intent_reader.read() {
//         match intent {
//             CursorIntentMessage::GoBack => {
//                 if let Some(entity) = grid.map.get(&cursor.get_rewind_pos())
//                     && let Ok((e, dir, pos)) = query.get_mut(*entity)
//                 {
//                     cursor.rewind(*dir);
//                     cursor.history.pop();
//                     grid.map.remove(pos);
//                     commands.entity(e).despawn();
//                 }
//             }
//             CursorIntentMessage::Print(key_code) => {
//                 if let Some(c) = key_code.as_char() {
//                     let entity =
//                         spawn_character(c, cursor.position, cursor.direction, &mut commands);
//                     grid.map.insert(cursor.position, entity);
//                     cursor.history.push(Character::new(c));
//
//                     if let Some(dir) = cursor.find_direction_match() {
//                         cursor.direction.transition(dir);
//                     }
//                     cursor.move_forward();
//                 }
//             }
//         }
//     }
// }
//
// fn spawn_character(
//     char: char,
//     position: Position,
//     direction: Direction,
//     commands: &mut Commands,
// ) -> Entity {
//     let ec = commands.spawn((Character::new(char), position, direction));
//     ec.id()
// }
//
// // fn match_state(history: Vec<Position, DirectionState>) -> Option<DirectionState> {
// //
// //     None
// // }
//
// // #[derive(Resource)]
// // pub enum TeleportState {
// //     NotTeleporting,
// //     IsTeleporting,
// //     TeleportEnding,
// // }
//
// // #[derive(Resource)]
// // pub enum JumpState {
// //
// // }

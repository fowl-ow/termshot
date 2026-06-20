use bevy::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

use crate::event::KeyEventMessage;

pub struct TermshotInputPlugin;

impl Plugin for TermshotInputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<CursorIntentMessage>();
        app.add_systems(PreUpdate, process_key_events);
    }
}

#[derive(Resource, Debug, Default)]
pub struct KeyInputHistory(Vec<KeyInput>);

#[derive(Debug)]
pub enum KeyInput {
    KeyPress(KeyCode),
    Backspace,
    Other,
}

// #[derive(Message)]
// pub enum CursorIntentMessage {
//     GoBack,
//     Print(KeyCode),
// }

#[expect(unused_variables)]
pub fn process_key_events(
    mut key_event_reader: MessageReader<KeyEventMessage>,
    mut exit_writer: MessageWriter<AppExit>,
    mut key_press_writer: MessageWriter<CursorIntentMessage>,
) {
    for message in key_event_reader.read() {
        match message.0 {
            KeyEvent {
                code: KeyCode::Char('q'),
                kind: crossterm::event::KeyEventKind::Press,
                modifiers,
                state,
            } => {
                exit_writer.write(AppExit::Success);
            }
            KeyEvent {
                code: KeyCode::Backspace,
                kind: crossterm::event::KeyEventKind::Press,
                modifiers,
                state,
            } => {
                key_press_writer.write(CursorIntentMessage::GoBack);
            }
            KeyEvent {
                code,
                kind: crossterm::event::KeyEventKind::Press,
                modifiers,
                state,
            } => {
                key_press_writer.write(CursorIntentMessage::Print(code));
            }
            KeyEvent {
                code,
                modifiers,
                kind,
                state,
            } => {}
        }
    }
}

use bevy::{
    app::{AppExit, Plugin, PreUpdate},
    ecs::{
        message::{Message, MessageReader, MessageWriter},
        system::{Commands, If, In},
    },
};
use crossterm::event::{KeyCode, KeyEvent};

use crate::render::Renderer;

pub struct TermshotInputPlugin;

impl Plugin for TermshotInputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<KeyEventMessage>();
        app.add_systems(PreUpdate, process_key_events);
    }
}

#[derive(Message)]
pub struct KeyEventMessage(pub KeyEvent);

#[expect(unused_variables)]
pub fn process_key_events(
    mut key_event_reader: MessageReader<KeyEventMessage>,
    mut exit_writer: MessageWriter<AppExit>,
) {
    for message in key_event_reader.read() {
        match message.0 {
            KeyEvent {
                code: KeyCode::Char('q'),
                kind,
                modifiers,
                state,
            } => {
                exit_writer.write(AppExit::Success);
            }
            KeyEvent {
                code: KeyCode::Char(' '),
                kind,
                modifiers,
                state,
            } => {
                // Renderer::Hello => commands.insert_resource(Renderer::Entities),
                // Renderer::Entities => commands.insert_resource(Renderer::Hello),
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

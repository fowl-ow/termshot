use std::time::Duration;

use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        message::{Message, MessageWriter},
        system::{Commands, In, IntoSystem, ResMut},
    },
};
use crossterm::event::{Event, KeyEvent, poll, read};

use crate::terminal::BufferSize;

pub struct TermshotTerminalEventPlugin;

impl Plugin for TermshotTerminalEventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PreUpdate, process_events.pipe(process_event_error_handler));
        app.add_message::<KeyEventMessage>();
    }
}

#[derive(Message)]
pub struct KeyEventMessage(pub KeyEvent);

pub fn process_events(
    mut buff_size: ResMut<BufferSize>,
    mut key_events_writer: MessageWriter<KeyEventMessage>,
) -> anyhow::Result<()> {
    while poll(Duration::from_millis(0))? {
        match read()? {
            Event::Resize(cols, rows) => {
                buff_size.cols = cols;
                buff_size.rows = rows;
            }
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key_event) => {
                key_events_writer.write(KeyEventMessage(key_event));
            }
            Event::Mouse(_mouse_event) => {}
            Event::Paste(_) => {}
        }
    }
    Ok(())
}

pub fn process_event_error_handler(In(result): In<Result<(), anyhow::Error>>) {
    if result.is_err() {}
}

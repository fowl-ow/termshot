use bevy::prelude::*;
use crossterm::event::{Event, KeyEvent, poll, read};
use std::time::Duration;

use crate::terminal::BufferSize;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, process_events.pipe(process_event_error_handler));
}

#[derive(Resource, Debug, Default)]
pub struct KeyEventHistory(Vec<KeyEvent>);

#[derive(Resource, Debug, Default)]
pub struct KeyEvents(Vec<KeyEvent>);

pub fn process_events(
    mut buff_size: ResMut<BufferSize>,
    mut key_event_history: ResMut<KeyEventHistory>,
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

use std::time::Duration;

use bevy_ecs::system::{Commands, In, ResMut};
use crossterm::event::{Event, poll, read};

use crate::{BufferSize, render::Renderer};

pub fn process_events(
    mut buff_size: ResMut<BufferSize>,
    mut commands: Commands,
    mut renderer: ResMut<Renderer>,
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
                crate::input::process_key_events(key_event, &mut commands, &mut renderer)
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

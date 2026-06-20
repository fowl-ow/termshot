use bevy::prelude::*;
use crossterm::event::{Event, KeyEvent, poll, read};
use std::time::Duration;

use crate::terminal::BufferSize;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedPreUpdate,
        process_events.pipe(process_event_error_handler),
    )
    .init_resource::<KeyEvents>()
    .init_resource::<KeyEventHistory>();
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct KeyEventHistory(Vec<KeyEvent>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct KeyEvents(Vec<KeyEvent>);

pub fn process_events(
    mut buff_size: ResMut<BufferSize>,
    mut key_event_history: ResMut<KeyEventHistory>,
    mut key_events: ResMut<KeyEvents>,
) -> anyhow::Result<()> {
    key_events.clear();
    while poll(Duration::from_millis(0))? {
        match read()? {
            Event::Resize(cols, rows) => {
                buff_size.cols = cols;
                buff_size.rows = rows;
            }
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Key(key_event) => {
                key_events.push(key_event);
            }
            Event::Mouse(_mouse_event) => {}
            Event::Paste(_) => {}
        }
    }
    key_event_history.extend_from_slice(key_events.as_ref());
    Ok(())
}

pub fn process_event_error_handler(In(result): In<Result<(), anyhow::Error>>) {
    if result.is_err() {
        // error!(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, super::plugin));
        app
    }

    #[test]
    fn test() {
        let mut app = setup_app();
        app.update();
    }
}

use bevy::{ecs::system::command::init_resource, prelude::*};
use crossterm::event::{Event, KeyEvent, poll, read};
use std::time::Duration;

use crate::terminal::BufferSize;

pub(super) fn terminal_event_plugin(app: &mut App) {
    app.add_systems(
        FixedPreUpdate,
        process_events.pipe(process_event_error_handler),
    )
    .init_resource::<KeyEvents>()
    .init_resource::<KeyEventHistory>()
    .insert_resource(EventSourceRes(Box::new(CrosstermEventSource)));
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct KeyEventHistory(Vec<KeyEvent>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct KeyEvents(Vec<KeyEvent>);

pub fn process_events(
    source: Res<EventSourceRes>,
    mut buff_size: ResMut<BufferSize>,
    mut key_event_history: ResMut<KeyEventHistory>,
    mut key_events: ResMut<KeyEvents>,
) -> anyhow::Result<()> {
    key_events.clear();
    while source.0.poll()? {
        match source.0.read()? {
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

trait EventSource: Send + Sync + 'static {
    fn poll(&self) -> anyhow::Result<bool>;
    fn read(&self) -> anyhow::Result<Event>;
}

struct CrosstermEventSource;

impl EventSource for CrosstermEventSource {
    fn poll(&self) -> anyhow::Result<bool> {
        Ok(poll(Duration::from_millis(0))?)
    }

    fn read(&self) -> anyhow::Result<Event> {
        Ok(read()?)
    }
}

#[derive(Resource)]
struct EventSourceRes(Box<dyn EventSource>);

#[cfg(test)]
mod test {
    use std::{collections::VecDeque, sync::Mutex};

    use crossterm::event::{KeyCode, KeyModifiers};

    use super::*;

    struct FakeEventSource(Mutex<VecDeque<Event>>);

    impl FakeEventSource {
        fn new(events: Vec<Event>) -> Self {
            Self(Mutex::new(events.into()))
        }
    }

    impl EventSource for FakeEventSource {
        fn poll(&self) -> anyhow::Result<bool> {
            Ok(self.0.lock().unwrap().is_empty())
        }

        fn read(&self) -> anyhow::Result<Event> {
            self.0
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| anyhow::anyhow!("no events"))
        }
    }

    #[test]
    fn test() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, terminal_event_plugin));
        let events: Vec<Event> = vec![
            Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
            Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        ];

        app.insert_resource(EventSourceRes(Box::new(FakeEventSource::new(events))));
    }
}

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
pub(crate) struct KeyEventHistory(Vec<KeyEvent>);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub(crate) struct KeyEvents(Vec<KeyEvent>);

fn process_events(
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

    use bevy::time::TimeUpdateStrategy;
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
            Ok(!self.0.lock().unwrap().is_empty())
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
    fn test_key_event_ressource_are_filled() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, terminal_event_plugin));
        app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));
        app.insert_resource(BufferSize {
            cols: 80,
            rows: 120,
        });

        let events: Vec<Event> = vec![
            Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
            Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
            Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        ];

        app.insert_resource(EventSourceRes(Box::new(FakeEventSource::new(
            events.clone(),
        ))));

        assert!(
            app.world()
                .get_resource::<KeyEventHistory>()
                .unwrap()
                .is_empty()
        );
        assert!(app.world().get_resource::<KeyEvents>().unwrap().is_empty());

        app.update();
        app.update();

        let key_events: Vec<KeyEvent> = events
            .iter()
            .map(|e: &Event| -> KeyEvent { e.as_key_event().unwrap() })
            .collect();

        assert_eq!(
            app.world().get_resource::<KeyEvents>().unwrap().0,
            key_events
        );
        assert_eq!(
            app.world().get_resource::<KeyEventHistory>().unwrap().0,
            key_events
        );

        app.insert_resource(EventSourceRes(Box::new(FakeEventSource::new(
            events.clone(),
        ))));

        app.update();

        assert_eq!(
            app.world().get_resource::<KeyEvents>().unwrap().0,
            key_events
        );
        assert_eq!(
            app.world().get_resource::<KeyEventHistory>().unwrap().0,
            // key_events.clone().extend_from_slice(&key_events)
            key_events
                .iter()
                .chain(key_events.iter())
                .cloned()
                .collect::<Vec<KeyEvent>>()
        );
    }
}

use bevy::{
    ecs::system::entity_command::clear, input::keyboard::KeyCode::KeyI,
    platform::collections::HashMap, prelude::*,
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{InputSystems, terminal_event::KeyEvents};

pub(super) fn input_plugin(app: &mut App) {
    // app.add_systems(FixedUpdate, system.in_set(IntentSystems));
    app.init_resource::<KeyInputs>();
    app.add_systems(
        FixedPreUpdate,
        key_event_to_key_input_system.in_set(InputSystems),
    );
}

fn key_event_to_key_input_system(key_events: Res<KeyEvents>, mut key_inputs: ResMut<KeyInputs>) {
    key_inputs.0.clear();
    key_inputs.0 = key_events.iter().filter_map(|e| e.to_key_input()).collect();
}

#[derive(Resource, Default)]
pub(crate) struct KeyInputs(Vec<KeyInput>);

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum KeyInput {
    GoBack,
    Char(char),
}

trait KeyEventToInput {
    fn to_key_input(self) -> Option<KeyInput>;
}

impl KeyEventToInput for KeyEvent {
    #[expect(unused_variables)]
    fn to_key_input(self) -> Option<KeyInput> {
        match self {
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers,
                kind,
                state,
            } => Some(KeyInput::GoBack),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                kind,
                state,
            } => Some(KeyInput::Char(c)),
            KeyEvent {
                code,
                modifiers,
                kind,
                state,
            } => None,
        }
    }
}

#[cfg(test)]
mod test {
    use bevy::time::TimeUpdateStrategy;
    use crossterm::event::KeyCode;
    use crossterm::event::KeyModifiers;

    use super::*;

    #[test]
    fn key_events_are_correctly_transformed_to_key_inputs() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, input_plugin));
        app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));

        let key_events = vec![
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        ];
        app.insert_resource(KeyEvents(key_events));

        let key_inputs = vec![KeyInput::Char('q'), KeyInput::GoBack, KeyInput::Char('a')];

        app.update();
        app.update();

        assert_eq!(
            app.world().get_resource::<KeyInputs>().unwrap().0,
            key_inputs
        );
    }
}

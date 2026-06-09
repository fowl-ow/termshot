use bevy_ecs::system::Commands;
use crossterm::event::{KeyCode, KeyEvent};

use crate::Exit;

#[expect(unused_variables)]
pub fn process_key_events(key_event: KeyEvent, commands: &mut Commands) {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('q'),
            kind,
            modifiers,
            state,
        } => {
            commands.insert_resource(Exit);
        }
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
        } => {}
    }
}

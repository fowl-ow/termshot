use bevy_ecs::system::Commands;
use crossterm::event::{KeyCode, KeyEvent};

use crate::Exit;
use crate::render::Renderer;

#[expect(unused_variables)]
pub fn process_key_events(key_event: KeyEvent, commands: &mut Commands, renderer: &mut Renderer) {
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
            code: KeyCode::Char(' '),
            kind,
            modifiers,
            state,
        } => match renderer {
            Renderer::Hello => commands.insert_resource(Renderer::Entities),
            Renderer::Entities => commands.insert_resource(Renderer::Hello),
        },
        KeyEvent {
            code,
            modifiers,
            kind,
            state,
        } => {}
    }
}

use bevy::{
    app::{Plugin, Startup},
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    input::gamepad::GamepadButton::Start,
};

use crate::terminal::BufferSize;

pub struct TermshotGameplayPlugin;

impl Plugin for TermshotGameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_random_components);
    }
}

#[derive(Debug, Component)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

fn spawn_random_components(mut commands: Commands, buf_size: Res<BufferSize>) {
    for i in 1..=100 {
        commands.spawn(Position { col: i * i, row: i });
    }
}

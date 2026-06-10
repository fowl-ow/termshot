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
        // app.add_systems(Startup, spawn_random_components);
    }
}

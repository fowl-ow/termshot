#![expect(unused_imports)]

use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

mod components;
mod game;
mod input;
mod map;
mod player_intent;
mod render;
mod resources;
mod state;
mod terminal;
mod terminal_event;
mod time;

pub struct TermshotPlugin;

impl Plugin for TermshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(50)),
            (
                terminal::terminal_plugin,
                terminal_event::terminal_event_plugin,
                time::time_plugin,
                game::game_plugin,
            ),
        ))
        .configure_sets(
            FixedPreUpdate,
            ((TerminalEventSystems, InputSystems).chain(), TimeSystems),
        )
        .configure_sets(
            FixedUpdate,
            (
                IntentSystems,
                ValidationSystems,
                ExecutionSystems,
                ReactionSystems,
            )
                .chain(),
        )
        .configure_sets(FixedPostUpdate, CleanupSystems)
        .configure_sets(PostUpdate, RenderSystems);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct TerminalEventSystems;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InputSystems;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct TimeSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct IntentSystems;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ValidationSystems;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ExecutionSystems;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ReactionSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct CleanupSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct RenderSystems;

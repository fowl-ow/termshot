#![expect(unused_imports)]

use std::{io, process::exit, thread::sleep, time::Duration};

use anyhow::Result;

use bevy::{
    MinimalPlugins,
    app::{App, AppExit, PluginGroup, ScheduleRunnerPlugin, Startup, Update},
    ecs::{
        component::Component,
        resource::Resource,
        schedule::{IntoScheduleConfigs, Schedule},
        system::{Commands, IntoSystem, Res},
    },
};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute, queue,
    style::Print,
    terminal::{
        Clear, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize,
        SetTitle, disable_raw_mode, enable_raw_mode, size,
    },
};

use crate::{
    event::TermshotTerminalEventPlugin, game::TermshotGameplayPlugin,
    key_input::TermshotInputPlugin, render::TermshotRenderPlugin, state::TermshotGameStatePlugin,
    terminal::TermshotTerminalPlugin,
};

mod cursor;
mod event;
mod game;
mod key_input;
mod map;
mod render;
mod state;
mod terminal;

pub fn app() -> Result<()> {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1 / 12))),
            (
                TermshotTerminalPlugin,
                TermshotTerminalEventPlugin,
                TermshotGameStatePlugin,
                TermshotGameplayPlugin,
                TermshotInputPlugin,
                TermshotRenderPlugin,
            ),
        ))
        .run();
    Ok(())
}

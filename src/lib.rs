#![expect(unused_imports)]

use std::{io, process::exit, thread::sleep, time::Duration};

use anyhow::Result;

use bevy::{
    MinimalPlugins,
    app::{App, PluginGroup, ScheduleRunnerPlugin, Startup, Update},
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
    event::TermshotTerminalEventPlugin, game::TermshotGameplayPlugin, input::TermshotInputPlugin,
    render::TermshotRenderPlugin, terminal::TermshotTerminalPlugin,
};

mod event;
mod game;
mod input;
mod render;
mod terminal;

pub fn app() -> Result<()> {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1 / 120))),
            (
                TermshotTerminalPlugin,
                TermshotTerminalEventPlugin,
                TermshotGameplayPlugin,
                TermshotInputPlugin,
                TermshotRenderPlugin,
            ),
        ))
        .run();
    Ok(())
}

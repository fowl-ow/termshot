#![expect(unused_imports)]

use std::{io, process::exit, thread::sleep, time::Duration};

use anyhow::Result;

use bevy::{
    DefaultPlugins, MinimalPlugins,
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
    key_input::TermshotInputPlugin, map::SpatialGridPlugin, render::TermshotRenderPlugin,
    state::TermshotGameStatePlugin, terminal::TermshotTerminalPlugin,
};

pub fn app() -> Result<()> {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ScheduleRunnerPlugin::run_loop(Duration::from_millis(50)),
            (
                TermshotTerminalPlugin,
                TermshotTerminalEventPlugin,
                TermshotGameStatePlugin,
                SpatialGridPlugin,
                TermshotGameplayPlugin,
                TermshotInputPlugin,
                TermshotRenderPlugin,
            ),
        ))
        .run();
    Ok(())
}

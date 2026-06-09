#![expect(unused_imports)]

use std::{io, process::exit, thread::sleep, time::Duration};

use anyhow::Result;

use bevy_ecs::{prelude::*, schedule};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    style::Print,
    terminal::{
        Clear, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize, SetTitle,
        disable_raw_mode, enable_raw_mode, size,
    },
};

mod input;
mod render;

static TITLE: &str = "Termshot";

#[derive(Resource)]
struct Exit;

pub fn app() -> Result<()> {
    let (cols, rows) = setup_terminal()?;

    let (mut world, mut schedule) = setup_bevy();

    loop {
        schedule.run(&mut world);
        // collect Input and write to ressource
        // collect deltatime and write to ressource
        // wait a certain amount of time
        // maybe check for exit resource?

        world.insert_resource::<Exit>(Exit);

        if world.get_resource::<Exit>().is_some() {
            clean_up_terminal()?;
            exit(0);
        };
    }
}

fn setup_bevy() -> (World, Schedule) {
    let mut world = World::new();
    let mut schedule = Schedule::default();

    (world, schedule)
}

fn setup_terminal() -> Result<(u16, u16)> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        SetTitle(TITLE),
        // Hide,
        MoveTo(0, 0),
        Clear(crossterm::terminal::ClearType::All)
    )?;
    let (cols, rows) = size()?;

    Ok((cols, rows))
}

fn clean_up_terminal() -> Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

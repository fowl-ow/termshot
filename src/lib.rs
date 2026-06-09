#![expect(unused_imports)]

use std::{io, process::exit, thread::sleep, time::Duration};

use anyhow::Result;

use bevy_ecs::{prelude::*, schedule};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute, queue,
    style::Print,
    terminal::{
        Clear, EnterAlternateScreen, LeaveAlternateScreen, ScrollUp, SetSize, SetTitle,
        disable_raw_mode, enable_raw_mode, size,
    },
};

mod event;
mod input;
mod render;

static TITLE: &str = "Termshot";

#[derive(Debug, Resource)]
struct Exit;

#[derive(Debug, Resource)]
struct BufferSize {
    cols: u16,
    rows: u16,
}

pub fn app() -> Result<()> {
    let (cols, rows) = setup_terminal()?;
    let buffer_size = BufferSize { cols, rows };
    let (mut world, mut schedule) = setup_bevy(buffer_size);

    loop {
        schedule.run(&mut world);

        if world.get_resource::<Exit>().is_some() {
            clean_up_terminal()?;
            exit(0);
        };
    }
}

fn setup_bevy(buffer_size: BufferSize) -> (World, Schedule) {
    let mut world = World::new();
    world.insert_resource(buffer_size);

    let mut schedule = Schedule::default();

    schedule.add_systems(
        (
            crate::event::process_events.pipe(crate::event::process_event_error_handler),
            render::render.pipe(render::error_handler_system),
        )
            .chain(),
    );

    (world, schedule)
}

fn setup_terminal() -> Result<(u16, u16)> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        SetTitle(TITLE),
        Hide,
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

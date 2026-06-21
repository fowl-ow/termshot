use std::io;

use bevy::prelude::*;

use crossterm::{
    cursor::{EnableBlinking, Hide, MoveTo, SetCursorStyle, Show},
    execute,
    terminal::{
        Clear, EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode,
        enable_raw_mode, size,
    },
};

static TITLE: &str = "Termshot";

#[derive(Debug, Resource)]
pub struct BufferSize {
    pub cols: u16,
    pub rows: u16,
}

pub(super) fn terminal_plugin(app: &mut bevy::app::App) {
    let (cols, rows) =
        size().expect("Panicking in TerminalPlugin Setup: Buffer size can't be determined!");
    app.insert_resource(BufferSize { cols, rows });
    app.add_systems(Startup, setup_terminal.pipe(handle_terminal_errors));
    app.add_systems(Last, clean_up_terminal.pipe(handle_terminal_errors));
}

fn setup_terminal() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        SetTitle(TITLE),
        MoveTo(0, 0),
        SetCursorStyle::BlinkingUnderScore,
        EnableBlinking,
        Clear(crossterm::terminal::ClearType::All)
    )?;
    Ok(())
}

fn clean_up_terminal(mut reader: MessageReader<AppExit>) -> anyhow::Result<()> {
    if reader.read().next().is_some() {
        execute!(io::stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
    }
    Ok(())
}

fn handle_terminal_errors(In(result): In<Result<(), anyhow::Error>>) {
    if let Err(e) = result {
        bevy::log::error!("{}", e);
    }
}

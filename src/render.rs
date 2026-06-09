use std::io::{Write, stdout};

use bevy_ecs::{
    resource::Resource,
    system::{Commands, In, Res},
};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::{BufferSize, Exit};

pub fn render(buff_size: Res<BufferSize>) -> anyhow::Result<()> {
    let BufferSize { cols, rows } = buff_size.as_ref();

    let center_col = cols / 2;
    let center_row = rows / 2;

    let start_col = center_col - 6;

    if *cols >= center_col + 6 {
        let mut out = stdout().lock();
        queue!(
            out,
            Clear(ClearType::All),
            MoveTo(0, 0),
            MoveTo(start_col, center_row),
            Print("hello there!")
        )?;
        out.flush()?;
    }

    Ok(())
}

pub fn error_handler_system(In(result): In<Result<(), anyhow::Error>>, mut commands: Commands) {
    if let Err(e) = result {
        std::fs::write("crash.log", format!("Render failed: {}", e)).unwrap();
        commands.insert_resource(Exit);
    }
}

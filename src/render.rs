use std::io::{Write, stdout};

use bevy::{
    app::{AppExit, Plugin, PostUpdate},
    ecs::{
        message::MessageWriter,
        resource::Resource,
        system::{Commands, In, IntoSystem, Query, Res},
    },
};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::{game::Position, terminal::BufferSize};

pub struct TermshotRenderPlugin;

impl Plugin for TermshotRenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Renderer::Hello).add_systems(
            PostUpdate,
            (
                render_positions.pipe(error_handler_system),
                render_hello_there.pipe(error_handler_system),
            ),
        );
    }
}

#[derive(Debug, Resource)]
pub enum Renderer {
    Hello,
    Entities,
}

pub fn render_positions(
    buff_size: Res<BufferSize>,
    query: Query<&Position>,
    renderer: Res<Renderer>,
) -> anyhow::Result<()> {
    match *renderer {
        Renderer::Hello => {}
        Renderer::Entities => {
            let BufferSize { cols, rows } = buff_size.as_ref();

            let mut out = stdout().lock();
            queue!(out, Clear(ClearType::All))?;
            for Position { col, row } in &query {
                if !(col >= cols || row >= rows) {
                    queue!(out, MoveTo(0, 0), MoveTo(*col, *row), Print("x"))?;
                }
            }
            out.flush()?;
        }
    }

    Ok(())
}

pub fn render_hello_there(
    buff_size: Res<BufferSize>,
    query: Query<&Position>,
    renderer: Res<Renderer>,
) -> anyhow::Result<()> {
    match *renderer {
        Renderer::Hello => {
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
        }
        Renderer::Entities => {}
    }
    Ok(())
}

pub fn error_handler_system(
    In(result): In<Result<(), anyhow::Error>>,
    mut writer: MessageWriter<AppExit>,
) {
    if let Err(e) = result {
        std::fs::write("crash.log", format!("Render failed: {}", e)).unwrap();
        writer.write(AppExit::error());
        bevy::log::error!("{}", e);
    }
}

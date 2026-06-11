use std::io::{Write, stdout};

use bevy::{
    app::{AppExit, Plugin, PostUpdate, Startup},
    ecs::{
        message::MessageWriter,
        query::{Added, Changed, Spawned},
        resource::Resource,
        schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
        system::{Commands, In, IntoSystem, Query, Res, SystemParamFunction},
    },
};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::{
    game::{Character, Cursor, Position},
    terminal::BufferSize,
};

pub struct TermshotRenderPlugin;

impl Plugin for TermshotRenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            PostUpdate,
            (
                // render_positions.pipe(error_handler_system),
                // render_hello_there.pipe(error_handler_system),
                render_characters.pipe(error_handler_system),
                // render_cursor.pipe(error_handler_system),
            ),
        );
        app.add_systems(PostUpdate, render_cursor.run_if(resource_changed::<Cursor>));
    }
}

fn render_characters(query: Query<(&Character, &Position), Spawned>) -> anyhow::Result<()> {
    let mut out = stdout().lock();
    queue!(out, SavePosition)?;
    for (char, pos) in query {
        let c = match char.char {
            ' ' => '_',
            other => other,
        };
        queue!(out, MoveTo(0, 0), MoveTo(pos.x, pos.y), Print(c))?;
    }
    queue!(out, RestorePosition)?;
    out.flush()?;
    Ok(())
}

fn render_cursor(cursor: Res<Cursor>, buff_size: Res<BufferSize>) {
    let mut out = stdout().lock();
    let _ = execute!(
        out,
        MoveTo(0, 0),
        MoveTo(cursor.position.x, cursor.position.y)
    );
    // if cursor.position.x <= buff_size.cols && cursor.position.y <= buff_size.rows {
    //     let out = stdout().lock();
    // }
}

fn error_handler_system(
    In(result): In<Result<(), anyhow::Error>>,
    mut writer: MessageWriter<AppExit>,
) {
    if let Err(e) = result {
        std::fs::write("crash.log", format!("Render failed: {}", e)).unwrap();
        writer.write(AppExit::error());
        bevy::log::error!("{}", e);
    }
}

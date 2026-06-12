use std::io::{Write, stdout};

use bevy::input::gamepad::GamepadButton::Start;
use bevy::prelude::*;
use bevy::{
    app::{AppExit, Plugin, PostUpdate, Startup},
    ecs::{
        message::MessageWriter,
        query::{Added, Changed, Spawned},
        resource::Resource,
        schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
        system::{Commands, In, IntoSystem, Query, Res, ResMut, SystemParamFunction},
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

#[derive(Resource, Default)]
struct CurrentBuffer {
    val: Vec<char>,
}

#[derive(Resource, Default)]
struct PreviousBuffer {
    val: Vec<char>,
}

pub struct TermshotRenderPlugin;

impl Plugin for TermshotRenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(CurrentBuffer::default());
        app.insert_resource(PreviousBuffer::default());
        app.add_systems(Startup, setup_render_buffers);
        app.add_systems(
            Update,
            update_render_buffers.run_if(resource_changed::<BufferSize>),
        );
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

fn setup_render_buffers(mut commands: Commands, buff_size: Res<BufferSize>) {
    let total_cells = (buff_size.cols * buff_size.rows) as usize;

    commands.insert_resource(CurrentBuffer {
        val: vec![' '; total_cells],
    });

    commands.insert_resource(PreviousBuffer {
        val: vec![' '; total_cells],
    });
}

fn update_render_buffers(
    mut commands: Commands,
    buff_size: Res<BufferSize>,
    mut current_buffer: ResMut<CurrentBuffer>,
    mut previous_buffer: ResMut<PreviousBuffer>,
) {
}

fn render_characters(
    query: Query<(&Character, &Position)>,
    mut current_buffer: ResMut<CurrentBuffer>,
    mut previous_buffer: ResMut<PreviousBuffer>,
    buff_size: Res<BufferSize>,
) -> anyhow::Result<()> {
    let cols = buff_size.cols as usize;
    let rows = buff_size.rows as usize;

    current_buffer.val.fill(' ');

    for (char, pos) in &query {
        let x = pos.x as usize;
        let y = pos.y as usize;

        if x < cols && y < rows {
            let c = match char.char {
                ' ' => '_',
                other => other,
            };

            let index = y * cols + x;
            current_buffer.val[index] = c;
        }
    }

    let mut out = stdout().lock();
    queue!(out, SavePosition)?;

    (0..rows).into_iter().for_each(|y| {
        (0..cols).into_iter().for_each(|x| {
            let index = y * cols + x;
            let current_char = current_buffer.val[index];
            let previous_char = previous_buffer.val[index];

            if current_char != previous_char {
                queue!(out, MoveTo(x as u16, y as u16), Print(current_char));
            }
        });
    });
    queue!(out, RestorePosition)?;
    out.flush()?;

    previous_buffer.val.copy_from_slice(&current_buffer.val);
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

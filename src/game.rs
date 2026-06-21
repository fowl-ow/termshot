use std::task::Poll::Pending;
use std::{fmt::Display, ops::Deref};

use bevy::prelude::*;

use crate::components::{Cursor, Enemy, Position};
use crate::{map::SpatialGrid, state::GameState, terminal::BufferSize};

pub(super) fn plugin(app: &mut App) {
    // spawn cursor in poststartup so we can later first spawn other stuff in the map like walls and
    // then spawn the cursor where there is free space
    app.add_systems(PostStartup, (spawn_cursor, spawn_enemy).chain());
}

fn spawn_cursor(mut commands: Commands, buff_size: Res<BufferSize>) {
    let x = f32::from(buff_size.cols / 2);
    let y = f32::from(buff_size.rows / 2);

    commands.spawn((Cursor, Position { x, y }));
}

fn spawn_enemy(mut commands: Commands, cursor_position: Single<&Position, With<Cursor>>) {
    cursor_position.x;
    commands.spawn((
        Enemy,
        Position {
            x: cursor_position.x + 20.0,
            y: cursor_position.y,
        },
    ));
}

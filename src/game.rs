use std::task::Poll::Pending;
use std::{fmt::Display, ops::Deref};

use bevy::prelude::*;

use crate::components::{Cursor, Enemy, Position};
use crate::terminal::BufferSize;

pub(super) fn game_plugin(app: &mut App) {
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
    commands.spawn((
        Enemy,
        Position {
            x: cursor_position.x + 20.0,
            y: cursor_position.y,
        },
    ));
}
#[cfg(test)]
mod test {
    use approx::{assert_relative_eq, relative_eq};

    use super::*;

    #[test]
    fn cursor_and_enemy_get_spawned_with_correct_position() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, game_plugin));

        let cols = 24;
        let rows = 24;

        app.insert_resource(BufferSize { cols, rows });

        app.update();

        let world = app.world_mut();

        let cursor_pos = world
            .query_filtered::<&Position, With<Cursor>>()
            .single(world)
            .unwrap()
            .clone();

        let enemy_pos = world
            .query_filtered::<&Position, With<Enemy>>()
            .single(world)
            .unwrap()
            .clone();

        assert_relative_eq!(cursor_pos.x + 20.0, enemy_pos.x);
        assert_relative_eq!(cursor_pos.y, enemy_pos.y);
    }
}

use bevy::prelude::*;

use crate::{
    TimeSystems,
    components::{Dead, TimeToLive},
};

fn time_plugin(app: &mut App) {
    app.add_systems(FixedPreUpdate, advance_timer.in_set(TimeSystems));
}

fn advance_timer(time: Res<Time>, mut timers: Query<&mut TimeToLive>) {
    for mut ttl in timers.iter_mut() {
        ttl.0.tick(time.delta());
    }
}

fn apply_death(mut commands: Commands, mut timers: Query<(Entity, &TimeToLive)>) {
    for (entity, ttl) in timers.iter_mut() {
        if ttl.is_finished() {
            commands.entity(entity).insert(Dead);
        }
    }
}

use bevy::prelude::*;

use crate::{
    TimeSystems,
    components::{Dead, TimeToLive},
};

pub fn time_plugin(app: &mut App) {
    app.add_systems(
        FixedPreUpdate,
        (advance_timer, apply_death).chain().in_set(TimeSystems),
    );
}

fn advance_timer(time: Res<Time>, mut timers: Query<&mut TimeToLive>) {
    info!("in advance_timer");
    for mut ttl in timers.iter_mut() {
        ttl.0.tick(time.delta());
        info!("in advance_timer iter");
    }
}

fn apply_death(mut commands: Commands, mut timers: Query<(Entity, &TimeToLive)>) {
    for (entity, ttl) in timers.iter_mut() {
        if ttl.is_finished() {
            commands.entity(entity).insert(Dead);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Add, time::Duration};

    use bevy::time::TimeUpdateStrategy;

    use super::*;

    #[test]
    fn test_tick_and_death_systems() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, time_plugin));
        app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));

        let duration = Duration::from_millis(1);

        let ttl_entity = app
            .world_mut()
            .spawn(TimeToLive(Timer::new(duration, TimerMode::Once)))
            .id();

        // update twice because time starts counting only after first update
        app.update();
        app.update();

        assert!(app.world().get::<Dead>(ttl_entity).is_some());
    }
}

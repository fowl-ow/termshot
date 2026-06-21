use bevy::prelude::*;

use crate::{
    CleanupSystems, TimeSystems,
    components::{Dead, TimeToLive},
};

pub fn time_plugin(app: &mut App) {
    app.add_systems(
        FixedPreUpdate,
        (advance_timer, apply_death).chain().in_set(TimeSystems),
    )
    .add_systems(
        FixedPostUpdate,
        cleanup_dead_entities.in_set(CleanupSystems),
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

fn cleanup_dead_entities(mut commands: Commands, dead_entities: Query<Entity, With<Dead>>) {
    for e in dead_entities.iter() {
        commands.entity(e).despawn();
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Add, time::Duration};

    use bevy::time::TimeUpdateStrategy;

    use super::*;

    #[test]
    fn tick_death_and_cleanup_systems_work() {
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

        assert!(app.world().get_entity(ttl_entity).is_err());
    }
}

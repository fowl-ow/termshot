use std::ops::Deref;

use bevy::{
    ecs::{event::Trigger, query::Spawned, relationship::RelationshipSourceCollection},
    platform::collections::HashMap,
    prelude::*,
};

use crate::game::{Character, PendingDespawn, Position};

pub struct SpatialGridPlugin;

impl Plugin for SpatialGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialGrid::default());
        // app.add_systems(PreUpdate, add_to_grid);
        // app.add_systems(PostUpdate, clean_up_from_grid);
        // app.add_systems(Last, clean_up);
    }
}

#[derive(Resource, Default)]
pub struct SpatialGrid {
    pub map: HashMap<Position, Entity>,
}

// fn add_to_grid(
//     mut grid: ResMut<SpatialGrid>,
//     query: Query<(Entity, &Position), (Spawned, With<Character>)>,
// ) {
//     for (entity, position) in query {
//         grid.map.insert(*position, entity);
//     }
// }
//
// fn clean_up_from_grid(
//     query: Query<&Position, With<PendingDespawn>>,
//     mut grid: ResMut<SpatialGrid>,
// ) {
//     query.iter().for_each(|position| {
//         grid.map.remove(position);
//     });
// }
//
// fn clean_up(query: Query<Entity, With<PendingDespawn>>) {
//     query.iter().for_each(|mut entity| {
//         entity.remove(entity);
//     });
// }

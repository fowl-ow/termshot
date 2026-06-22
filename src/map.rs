use bevy::prelude::*;

use crate::{components::Position, resources::PositionMap};

pub(super) fn position_map_plugin(app: &mut App) {
    app.init_resource::<PositionMap>();
    app.add_systems(FixedPreUpdate, build_position_map);
}

fn build_position_map(mut position_map: ResMut<PositionMap>, entities: Query<(Entity, &Position)>) {
    for (entity, position) in entities.iter() {
        position_map.0.insert(*position, entity);
    }
}

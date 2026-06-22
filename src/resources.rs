use bevy::{platform::collections::HashMap, prelude::*};

use crate::components::Position;

#[derive(Resource, Debug, Default, DerefMut, Deref)]
pub(crate) struct PositionMap(pub HashMap<Position, Entity>);

#[derive(Resource, Debug)]
pub(crate) struct CommandHistory(Vec<u32>);

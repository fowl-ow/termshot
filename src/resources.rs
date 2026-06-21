use bevy::{platform::collections::HashMap, prelude::*};

use crate::components::Position;

#[derive(Resource, Debug)]
struct PositionMap(HashMap<Position, Entity>);

#[derive(Resource, Debug)]
struct InputHistory(Vec<u32>);

#[derive(Resource, Debug)]
struct CommandHistory(Vec<u32>);

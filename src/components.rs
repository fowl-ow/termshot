use std::time::Duration;

use better_default::Default;
use bevy::{prelude::*, time::Timer};

#[derive(Component, Debug, Default)]
pub(crate) struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default)]
#[default(0: 3)]
pub(crate) struct Health(u8);

#[derive(Component, Debug, Default, Deref, DerefMut)]
#[default(0: Timer::new(Duration::from_secs(15), TimerMode::Once))]
pub(crate) struct TimeToLive(pub Timer);

#[derive(Component, Debug, Default)]
#[default(0: 1.0)]
pub(crate) struct Speed(f32);

#[derive(Component, Debug, Default)]
pub(crate) struct BlastRadius;

#[derive(Component, Debug)]
#[require(Position, Health)]
pub(crate) struct Cursor;

#[derive(Component, Debug)]
#[require(Position, Health)]
pub(crate) struct Enemy;

#[derive(Component, Debug)]
#[require(Position, TimeToLive)]
pub(crate) struct Character(char);

#[derive(Component, Debug)]
#[require(Position, Speed, BlastRadius)]
pub(crate) struct Missile;

#[derive(Component, Debug)]
#[require(Position)]
pub(crate) struct Wall;

#[derive(Component, Debug)]
#[require(Position, Speed)]
pub(crate) struct Bullet;

#[derive(Component, Debug)]
pub(crate) struct Dead;

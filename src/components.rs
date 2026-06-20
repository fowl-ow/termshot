use better_default::Default;
use bevy::prelude::*;

#[derive(Component, Debug, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, Default)]
#[default(0: 3)]
struct Health(u8);

#[derive(Component, Debug, Default)]
struct TimeToLive(Timer);

#[derive(Component, Debug, Default)]
#[default(0: 1.0)]
struct Speed(f32);

#[derive(Component, Debug, Default)]
struct BlastRadius;

#[derive(Component, Debug)]
#[require(Position, Health)]
struct Cursor;

#[derive(Component, Debug)]
#[require(Position, Health)]
struct Enemy;

#[derive(Component, Debug)]
#[require(Position, TimeToLive)]
struct Character(char);

#[derive(Component, Debug)]
#[require(Position, Speed, BlastRadius)]
struct Missile;

#[derive(Component, Debug)]
#[require(Position)]
struct Wall;

#[derive(Component, Debug)]
#[require(Position, Speed)]
struct Bullet;

#[derive(Component, Debug)]
struct Dead;

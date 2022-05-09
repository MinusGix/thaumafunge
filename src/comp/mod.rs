use bevy::{math::IVec2, prelude::Component};

pub mod ai;
pub mod beings;
pub mod display;
pub mod entity;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Component)]
pub struct Position(pub IVec2);
impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position(IVec2::new(x, y))
    }
}

#[derive(Component)]
pub struct Walkable;

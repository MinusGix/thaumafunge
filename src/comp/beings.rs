use bevy::prelude::*;

use super::{
    ai::RandomAI,
    display::Renderable,
    entity::{Being, TurnTaker},
    Position,
};

#[derive(Bundle)]
pub struct BasicMonsterBundle<AI: Component> {
    pub being: Being,
    pub position: Position,
    #[bundle]
    pub renderable: Renderable,
    pub turn_taker: TurnTaker,
    pub ai: AI,
}

pub fn new_zombie(atlas: Handle<TextureAtlas>, position: Position) -> BasicMonsterBundle<RandomAI> {
    BasicMonsterBundle {
        being: Being {},
        position,
        renderable: Renderable::from_index(atlas, 2, Color::GRAY),
        turn_taker: TurnTaker::new(30),
        ai: RandomAI::default(),
    }
}

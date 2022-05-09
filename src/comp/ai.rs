use bevy::prelude::Component;

use crate::rng::Random;

#[derive(Component)]
pub struct PlayerAI;

#[derive(Component, Default)]
pub struct RandomAI {
    pub rand: Random,
}

pub struct FactionDisposition<const DISP: Disposition, const FACTION: Faction> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// They will work with the others and help defend them
    /// Tends to be forgiving about accidental hits
    WorkWith,
    /// They won't attack unless the other side attacks
    Neutral,
    /// Will attack on sight
    Enemy,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Player,
    Undead,
}

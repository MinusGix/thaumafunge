use bevy::prelude::Component;

/// A marker for the entity that the current turn is for
#[derive(Component)]
pub struct ActiveTurn;

/// Something that can perform an action during a turn
/// Entitys typically have this
#[derive(Component)]
pub struct TurnTaker {
    pub max_energy: i32,
    pub energy: i32,
}
impl TurnTaker {
    pub fn new(max_energy: i32) -> TurnTaker {
        TurnTaker {
            max_energy,
            energy: max_energy,
        }
    }
}

/// A 'being' in the world
#[derive(Component)]
pub struct Being {}

#[derive(Component)]
pub struct Mana {
    pub max: u32,
    pub current: u32,
}
impl Mana {
    pub fn new(max: u32) -> Mana {
        Mana { max, current: max }
    }
}

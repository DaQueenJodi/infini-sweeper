use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub is_revealed: bool,
    pub num: u8,
    pub is_flagged: bool,
    pub is_mine: bool,
}

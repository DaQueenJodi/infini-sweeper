use bevy::prelude::*;
use rand::Rng;

use crate::{textures::{GameTextures, SPRITE_SCALE}, components::Tile};

pub const CHUNK_W: u8 = 10;
pub const CHUNK_H: u8 = 8;
pub const MINES_PER_CHUNK: usize = 10;


fn tile_spawn_chunk_system(
    mut commands: Commands,
    game_textures: GameTextures,
) {
    
    let rng = rand::thread_rng();

    let mut mine_positions: [(u8, u8); MINES_PER_CHUNK];

    for i in 0..MINES_PER_CHUNK {
        let x = rng.gen_range(0..CHUNK_W);
        let y = rng.gen_range(0..CHUNK_H);

        mine_positions[i] = (x, y);
    }

    
    let spawn_tile = |x: f32, y: f32| {

    
    let is_mine = mine_positions.contains(&(x as u8, y as u8));

     let mut tile = commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: game_textures.tiles,
        transform: Transform {
            scale: Vec3::from_array(SPRITE_SCALE),
            translation: Vec3::new(x, y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

     tile.insert(Tile {
        is_revealed: false,
        num: 0,
        is_mine,
        is_flagged: false,
     });

   
    };

    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            
        }
    }
    }

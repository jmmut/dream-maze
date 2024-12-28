use macroquad::prelude::UVec2;
use macroquad::rand::rand;
use std::ops::IndexMut;

pub type Coord = u32;
pub type Coord2 = UVec2;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Floor,
    Wall,
    // Coin,
    // Exit,
}
pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {}

impl Map {
    pub fn new(screen_tiles: Coord2, player: Coord2) -> Self {
        let mut tiles = Vec::new();
        for _i_x in 0..screen_tiles.x {
            let mut column = Vec::new();
            for _i_y in 0..screen_tiles.y {
                column.push(if rand() % 2 == 0 {
                    Tile::Wall
                } else {
                    Tile::Floor
                });
            }
            tiles.push(column);
        }
        let mut map = Self { tiles };
        *map.get_mut(player.x, player.y) = Tile::Floor;
        map
    }

    pub fn is_wall(&self, x: Coord, y: Coord) -> bool {
        self.get(x, y) == Tile::Wall
    }
    pub fn get(&self, x: Coord, y: Coord) -> Tile {
        self.tiles[x as usize][y as usize]
    }
    pub fn get_mut(&mut self, x: Coord, y: Coord) -> &mut Tile {
        self.tiles.index_mut(x as usize).index_mut(y as usize)
    }
}

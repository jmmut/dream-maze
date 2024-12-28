use macroquad::prelude::{IVec2, UVec2};
use macroquad::rand::rand;
use std::ops::IndexMut;

pub type Coord = u32;
pub type Coord2 = UVec2;
pub type CoordDiff = i32;
pub type CoordDiff2 = IVec2;

const DOWN: CoordDiff2 = CoordDiff2::new(0, 1);
const UP: CoordDiff2 = CoordDiff2::new(0, -1);
const LEFT: CoordDiff2 = CoordDiff2::new(-1, 0);
const RIGHT: CoordDiff2 = CoordDiff2::new(1, 0);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Floor,
    Wall,
    // Coin,
    // Exit,
}
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    offset: Coord2,
    player: Coord2,
}

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
        let offset = Coord2::new(0, 0);
        let mut map = Self {
            tiles,
            offset,
            player,
        };
        *map.get_mut(player) = Tile::Floor;
        map
    }

    pub fn is_wall(&self, x: Coord, y: Coord) -> bool {
        self.get(Coord2::new(x, y)) == Tile::Wall
    }

    pub fn move_down(&mut self) {
        self.move_to(DOWN);
    }
    pub fn move_up(&mut self) {
        self.move_to(UP);
    }
    pub fn move_right(&mut self) {
        self.move_to(RIGHT);
    }
    pub fn move_left(&mut self) {
        self.move_to(LEFT);
    }
    pub fn move_to(&mut self, diff: CoordDiff2) {
        if self.get_rel(self.player, diff) != Tile::Wall {
            self.offset = self.add_coord(self.offset, diff)
        }
    }
    pub fn get(&self, pos: Coord2) -> Tile {
        let Coord2 {
            x: size_x,
            y: size_y,
        } = self.size();
        self.get_raw(
            (pos.x + self.offset.x) % size_x,
            (pos.y + self.offset.y) % size_y,
        )
    }
}
impl Map {
    fn get_raw(&self, x: Coord, y: Coord) -> Tile {
        self.tiles[x as usize][y as usize]
    }
    fn get_rel(&self, pos: Coord2, diff: CoordDiff2) -> Tile {
        self.get(self.add_coord(pos, diff))
    }
    fn get_raw_mut(&mut self, x: Coord, y: Coord) -> &mut Tile {
        self.tiles.index_mut(x as usize).index_mut(y as usize)
    }
    fn get_mut(&mut self, pos: Coord2) -> &mut Tile {
        let Coord2 {
            x: size_x,
            y: size_y,
        } = self.size();
        self.get_raw_mut(
            (pos.x + self.offset.x) % size_x,
            (pos.y + self.offset.y) % size_y,
        )
    }
    fn size(&self) -> Coord2 {
        Coord2::new(self.tiles.len() as Coord, self.tiles[0].len() as Coord)
    }
    fn add_coord(&self, pos: Coord2, diff: CoordDiff2) -> Coord2 {
        let size = self.size();
        let unsigned_diff = Coord2::new(
            (diff.x + size.x as CoordDiff) as Coord,
            (diff.y + size.y as CoordDiff) as Coord,
        );
        (pos + unsigned_diff) % size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Map {
        fn get_offset(&self) -> Coord2 {
            self.offset
        }
    }
    #[test]
    fn test_offset_modulus_vertical() {
        let mut map = Map::new(Coord2::new(10, 10), Coord2::new(2, 5));
        assert_eq!(map.get_offset(), Coord2::new(0, 0));

        map.move_down();
        assert_eq!(map.get_offset(), Coord2::new(0, 1));
        map.move_down();
        assert_eq!(map.get_offset(), Coord2::new(0, 2));

        map.move_up();
        assert_eq!(map.get_offset(), Coord2::new(0, 1));
        map.move_up();
        assert_eq!(map.get_offset(), Coord2::new(0, 0));

        map.move_up();
        assert_eq!(map.get_offset(), Coord2::new(0, 9));
        map.move_up();
        assert_eq!(map.get_offset(), Coord2::new(0, 8));

        map.move_down();
        assert_eq!(map.get_offset(), Coord2::new(0, 9));
        map.move_down();
        assert_eq!(map.get_offset(), Coord2::new(0, 0));
    }
    #[test]
    fn test_offset_modulus_horizontal() {
        let mut map = Map::new(Coord2::new(10, 10), Coord2::new(2, 5));
        assert_eq!(map.get_offset(), Coord2::new(0, 0));

        map.move_right();
        assert_eq!(map.get_offset(), Coord2::new(1, 0));
        map.move_right();
        assert_eq!(map.get_offset(), Coord2::new(2, 0));

        map.move_left();
        assert_eq!(map.get_offset(), Coord2::new(1, 0));
        map.move_left();
        assert_eq!(map.get_offset(), Coord2::new(0, 0));

        map.move_left();
        assert_eq!(map.get_offset(), Coord2::new(9, 0));
        map.move_left();
        assert_eq!(map.get_offset(), Coord2::new(8, 0));

        map.move_right();
        assert_eq!(map.get_offset(), Coord2::new(9, 0));
        map.move_right();
        assert_eq!(map.get_offset(), Coord2::new(0, 0));
    }
}

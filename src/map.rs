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
    offset: Coord2,
}

impl Map {}

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
        let offset = Coord2::new(0, 0);
        let mut map = Self { tiles, offset };
        *map.get_mut(player.x, player.y) = Tile::Floor;
        map
    }

    pub fn is_wall(&self, x: Coord, y: Coord) -> bool {
        let Coord2 {
            x: size_x,
            y: size_y,
        } = self.size();
        let tile = self.get((x + self.offset.x) % size_x, (y + self.offset.y) % size_y);
        tile == Tile::Wall
    }

    pub fn move_down(&mut self) {
        self.offset.y += 1;
        self.offset.y %= self.size().y;
    }
    pub fn move_up(&mut self) {
        let size_y = self.size().y;
        self.offset.y += size_y - 1;
        self.offset.y %= self.size().y;
    }
    pub fn move_right(&mut self) {
        self.offset.x += 1;
        self.offset.x %= self.size().x;
    }
    pub fn move_left(&mut self) {
        let size_x = self.size().x;
        self.offset.x += size_x - 1;
        self.offset.x %= self.size().x;
    }
}
impl Map {
    fn get(&self, x: Coord, y: Coord) -> Tile {
        self.tiles[x as usize][y as usize]
    }
    fn get_mut(&mut self, x: Coord, y: Coord) -> &mut Tile {
        self.tiles.index_mut(x as usize).index_mut(y as usize)
    }
    fn size(&self) -> Coord2 {
        Coord2::new(self.tiles.len() as Coord, self.tiles[0].len() as Coord)
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

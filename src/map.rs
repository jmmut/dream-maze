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
    Monster,
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
                column.push(Self::generate_tile());
            }
            tiles.push(column);
        }
        let offset = Coord2::new(0, 0);
        let mut map = Self {
            tiles,
            offset,
            player,
        };
        for i_x in 0..screen_tiles.x {
            *map.get_mut(Coord2::new(i_x, player.y)) = Tile::Floor;
        }
        for i_y in 0..screen_tiles.y {
            *map.get_mut(Coord2::new(player.x, i_y)) = Tile::Floor;
        }
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
            for i_y in 0..diff.y {
                self.replace_row(i_y)
            }
            for i_y in 0..-diff.y {
                self.replace_row(self.size().y as CoordDiff - i_y - 1);
            }
            for i_x in 0..diff.x {
                self.replace_column(i_x)
            }
            for i_x in 0..-diff.x {
                self.replace_column(self.size().x as CoordDiff - i_x - 1);
            }
            self.offset = self.add_coord(self.offset, diff)
        }
    }

    fn replace_row(&mut self, i_y: i32) {
        assert!(self.in_range_y(i_y));
        let i_y = i_y as Coord;
        for i_x in 0..self.size().x {
            *self.get_mut(Coord2::new(i_x, i_y)) = Self::generate_tile();
        }
    }
    fn replace_column(&mut self, i_x: i32) {
        assert!(self.in_range_x(i_x));
        let i_x = i_x as Coord;
        for i_y in 0..self.size().y {
            *self.get_mut(Coord2::new(i_x, i_y)) = Self::generate_tile();
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
    fn in_range_y(&self, y: CoordDiff) -> bool {
        0 <= y && y < self.size().y as CoordDiff
    }
    fn in_range_x(&self, x: CoordDiff) -> bool {
        0 <= x && x < self.size().x as CoordDiff
    }
    fn generate_tile() -> Tile {
        let random = rand() % 100;
        if random < 49 {
            Tile::Wall
        } else if random < 98 {
            Tile::Floor
        } else {
            Tile::Monster
        }
    }
}

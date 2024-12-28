mod map;

use crate::map::Tile;
use crate::map::{Coord, Coord2, Map};
use macroquad::prelude::*;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Dream Maze";

type Pixels = f32;
type Pixels2 = Vec2;

// https://supercolorpalette.com/?scp=G0-hsl-E4A84E-B2DF49-45D945-41D2A7-3E93CC
const COLOR_BACKGROUND: Color = color_from_hex(0x3E93CCFF);
const COLOR_WALL: Color = color_from_hex(0xE4A84EFF);
const COLOR_PLAYER: Color = color_from_hex(0x45D945FF);
const COLOR_MONSTER: Color = color_from_hex(0x9F3DB8FF);

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand(42000);
    let tile_size = Pixels2::new(32.0, 32.0);
    let screen_tiles = pixel_to_tile(screen_width(), screen_height(), tile_size);
    println!("map size: {:?}", screen_tiles);
    let player = screen_tiles / 2;
    let mut map = Map::new(screen_tiles, player);

    loop {
        clear_background(LIGHTGRAY);
        let end_of_map = tile_to_pixel(screen_tiles.x, screen_tiles.y, tile_size);
        draw_rectangle(0.0, 0.0, end_of_map.x, end_of_map.y, COLOR_BACKGROUND);
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Down) {
            map.move_down()
        }
        if is_key_pressed(KeyCode::Up) {
            map.move_up()
        }
        if is_key_pressed(KeyCode::Left) {
            map.move_left()
        }
        if is_key_pressed(KeyCode::Right) {
            map.move_right()
        }
        if is_mouse_button_released(MouseButton::Left) {
            let click = Vec2::from(mouse_position());
            let clicked_tile = pixel_to_tile(click.x, click.y, tile_size);
            let tile = map.get(clicked_tile);
            println!("tile at {:?} is {:?}", clicked_tile, tile);
        }
        for i_x in 0..screen_tiles.x {
            for i_y in 0..screen_tiles.y {
                let tile = map.get(Coord2::new(i_x, i_y));

                let pixel = tile_to_pixel(i_x, i_y, tile_size);
                match tile {
                    Tile::Wall => {
                        draw_rectangle(pixel.x, pixel.y, tile_size.x, tile_size.y, COLOR_WALL)
                    }
                    Tile::Monster => {
                        let top = pixel + Vec2::new(tile_size.x * 0.5, tile_size.y * 0.2);
                        let left = pixel + Vec2::new(tile_size.x * 0.2, tile_size.y * 0.8);
                        let right = pixel + Vec2::new(tile_size.x * 0.8, tile_size.y * 0.8);
                        draw_triangle(top, left, right, COLOR_MONSTER);
                    }

                    _ => {}
                };
            }
        }

        let mut pixel = tile_to_pixel(player.x, player.y, tile_size);
        pixel += tile_size * 0.5; // circle position is the center
        draw_circle(pixel.x, pixel.y, 10.0, COLOR_PLAYER);

        next_frame().await
    }
}

fn pixel_to_tile(x: Pixels, y: Pixels, tile_size: Pixels2) -> Coord2 {
    Coord2::new((x / tile_size.x) as Coord, (y / tile_size.y) as Coord)
}
fn tile_to_pixel(x: Coord, y: Coord, tile_size: Pixels2) -> Pixels2 {
    Pixels2::new(x as Pixels * tile_size.x, y as Pixels * tile_size.y)
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

const fn color_from_hex(mut hex: u32) -> Color {
    let a = (hex & 0xFF) as u8;
    hex >>= 8;
    let b = (hex & 0xFF) as u8;
    hex >>= 8;
    let g = (hex & 0xFF) as u8;
    hex >>= 8;
    let r = (hex & 0xFF) as u8;
    color_from_rgba(r, g, b, a)
}

pub const fn color_from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(
        r as f32 / 255.,
        g as f32 / 255.,
        b as f32 / 255.,
        a as f32 / 255.,
    )
}

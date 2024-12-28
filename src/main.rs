use macroquad::prelude::*;
use macroquad::rand::rand;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Dream Maze";

type Pixels2 = Vec2;

// https://supercolorpalette.com/?scp=G0-hsl-E4A84E-B2DF49-45D945-41D2A7-3E93CC
const COLOR_BACKGROUND: Color = color_from_hex(0x3E93CCFF);
const COLOR_WALL: Color = color_from_hex(0xE4A84EFF);
const COLOR_PLAYER: Color = color_from_hex(0x45D945FF);

#[macroquad::main(window_conf)]
async fn main() {
    let tile_size = Pixels2::new(32.0, 32.0);
    let screen_tiles = IVec2::new(
        (screen_width() / tile_size.x) as i32,
        (screen_height() / tile_size.y) as i32,
    );
    let player = screen_tiles / 2;

    loop {
        macroquad::rand::srand(42000);
        clear_background(COLOR_BACKGROUND);
        if is_key_down(KeyCode::Escape) {
            break;
        }
        for i_x in 0..screen_tiles.x {
            for i_y in 0..screen_tiles.y {
                if rand() % 2 == 0 {
                    draw_rectangle(
                        i_x as f32 * tile_size.x,
                        i_y as f32 * tile_size.y,
                        tile_size.x,
                        tile_size.y,
                        COLOR_WALL,
                    )
                }
            }
        }
        draw_circle(
            (player.x as f32 + 0.5) * tile_size.x,
            (player.y as f32 + 0.5) * tile_size.y,
            10.0,
            COLOR_PLAYER,
        );

        next_frame().await
    }
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

fn random_color() -> Color {
    Color::from_rgba(
        (rand() % 256) as u8,
        (rand() % 256) as u8,
        (rand() % 256) as u8,
        255,
    )
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

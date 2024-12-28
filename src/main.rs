use macroquad::prelude::*;
use macroquad::rand::rand;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Dream Maze";

type Pixels2 = Vec2;

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand(42000);
    let tile_size = Pixels2::new(10.0, 10.0);
    let screen_tiles = IVec2::new(
        (screen_width() / tile_size.x) as i32,
        (screen_height() / tile_size.y) as i32,
    );

    loop {
        clear_background(LIGHTGRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }
        for i_x in 0..screen_tiles.x {
            for i_y in 0..screen_tiles.y {
                draw_rectangle(
                    i_x as f32 * tile_size.x,
                    i_y as f32 * tile_size.y,
                    tile_size.x,
                    tile_size.y,
                    random_color(),
                )
            }
        }

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

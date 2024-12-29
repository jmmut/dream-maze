mod map;

use crate::map::Tile;
use crate::map::{Coord, Coord2, Map};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Interaction, InteractionStyle, Style};
use juquad::widgets::text::TextRect;
use macroquad::prelude::*;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Dream Maze";

type Pixels = f32;
type Pixels2 = Vec2;

// https://supercolorpalette.com/?scp=G0-hsl-E4A84E-B2DF49-45D945-41D2A7-3E93CC-483BC4-9F3DB8-AB3F75
const COLOR_BACKGROUND: Color = color_from_hex(0x3E93CCFF);
const COLOR_WALL: Color = color_from_hex(0xE4A84EFF);
const COLOR_PLAYER: Color = color_from_hex(0x45D945FF);
const COLOR_MONSTER: Color = color_from_hex(0x9F3DB8FF);

const COLOR_UI_BG: Color = color_from_hex(0xf9e1ffFF);
const COLOR_UI_LIGHTER: Color = color_from_hex(0xCB9FD5FF);
const COLOR_UI: Color = color_from_hex(0x9C4CAEFF);
const COLOR_UI_DARKER: Color = color_from_hex(0x4F2759FF);
const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style {
    text_color: InteractionStyle {
        at_rest: COLOR_UI_BG,
        hovered: COLOR_UI_DARKER,
        pressed: COLOR_UI_LIGHTER,
    },
    bg_color: InteractionStyle {
        at_rest: COLOR_UI,
        hovered: COLOR_UI_LIGHTER,
        pressed: COLOR_UI_DARKER,
    },
    border_color: InteractionStyle {
        at_rest: COLOR_UI,
        hovered: COLOR_UI_DARKER,
        pressed: DARKGRAY,
    },
};

const MAX_HEALTH: f32 = 5.0;

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand(42000);
    let tile_size = Pixels2::new(32.0, 32.0);
    let screen_tiles = pixel_to_tile(screen_width(), screen_height(), tile_size);
    println!("map size: {:?}", screen_tiles);
    let player = screen_tiles / 2;
    let mut player_health = MAX_HEALTH;
    let mut map = Map::new(screen_tiles, player);
    let mut frame = 0;
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

        if (frame + 1) % 60 == 0 {
            map.advance();
            if map.get(map.player) == Tile::Monster {
                player_health = 0.0_f32.max(player_health - 1.0);
            }
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

        let health_unit: Pixels = 20.0;
        draw_rectangle(
            10.0,
            10.0,
            MAX_HEALTH * health_unit + 4.0,
            health_unit + 4.0,
            COLOR_UI_DARKER,
        );
        draw_rectangle(
            12.0,
            12.0,
            player_health * health_unit,
            health_unit,
            COLOR_PLAYER,
        );

        if player_health <= 0.0 {
            let window_width = 200.0;
            let window = Rect::new(
                screen_width() * 0.5 - window_width * 0.5,
                screen_height() * 0.4,
                window_width,
                150.0,
            );
            draw_rect(window, COLOR_UI_BG);
            draw_rect_lines(window, 2.0, COLOR_UI_DARKER);
            let text_anchor = Anchor::top_center(screen_width() * 0.5, screen_height() * 0.5);
            let text = TextRect::new("You died", text_anchor, FONT_SIZE);
            text.render_text(BLACK);
            let button_anchor = Anchor::center_below(text.rect, 0.0, 10.0);
            let mut retry = create_button("Retry", button_anchor);
            if retry.interact().is_clicked() {
                map = Map::new(screen_tiles, player);
                player_health = MAX_HEALTH;
            }
            retry.render(&STYLE);
        }
        frame = (frame + 1) % 10000;
        next_frame().await
    }
}

fn create_button(text: &str, anchor: Anchor) -> Button {
    Button::new_generic(
        text,
        anchor,
        FONT_SIZE,
        measure_text,
        draw_text,
        render_button,
        Box::new(InputMacroquad),
    )
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

pub fn render_button(interaction: Interaction, text_rect: &TextRect, style: &Style) {
    let (bg_color, text_color) = match interaction {
        Interaction::Clicked | Interaction::Pressing => {
            (style.bg_color.pressed, style.text_color.pressed)
        }
        Interaction::Hovered => (style.bg_color.hovered, style.text_color.hovered),
        Interaction::None => (style.bg_color.at_rest, style.text_color.at_rest),
    };
    let rect = text_rect.rect;
    draw_rect(rect, bg_color);
    draw_panel_border(rect, interaction, &style.border_color);
    text_rect.render_text(text_color);
}

pub fn draw_panel_border(rect: Rect, interaction: Interaction, style: &InteractionStyle) {
    let color = match interaction {
        Interaction::Clicked | Interaction::Pressing => style.pressed,
        Interaction::Hovered => style.hovered,
        Interaction::None => style.at_rest,
    };
    draw_rect_lines(rect, 2.0, color)
}

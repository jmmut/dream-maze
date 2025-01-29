mod map;

use crate::map::{Coord, Coord2, Map};
use crate::map::{CoordDiff, CoordDiff2, Tile, DOWN, LEFT, RIGHT, UP};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Interaction, InteractionStyle, Style};
use juquad::widgets::text::TextRect;
use macroquad::prelude::*;
use macroquad::rand::rand;

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;
const DEFAULT_WINDOW_TITLE: &str = "Dream Maze";

type Pixels = f32;
type Pixels2 = Vec2;

// https://supercolorpalette.com/?scp=G0-hsl-E4A84E-B2DF49-45D945-41D2A7-3E93CC-483BC4-9F3DB8-AB3F75
const COLOR_BACKGROUND: Color = color_from_hex(0x3E93CCFF);
const COLOR_WALL: Color = color_from_hex(0xE4A84EFF);
const COLOR_DOOR: Color = color_from_hex(0x7C351DFF);
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
const REQUIRED_DOORS: i32 = 4;

pub struct GameState {
    player_health: f32,
    map: Map,
    doors_parts_collected: i32,
}
impl GameState {
    pub fn new(screen_tiles: Coord2, player: Coord2) -> Self {
        Self {
            player_health: MAX_HEALTH,
            map: Map::new(screen_tiles, player),
            doors_parts_collected: 0,
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::rand::srand(42000);
    let tile_size = Pixels2::new(32.0, 32.0);
    let screen_tiles = pixel_to_tile(screen_width(), screen_height(), tile_size);
    println!("map size: {:?}", screen_tiles);
    let player = screen_tiles / 2;
    let mut game_state = GameState::new(screen_tiles, player);
    let mut accumulated_pos = CoordDiff2::new(0, 0);
    let mut next_door = calculate_rand_accumulated_pos(accumulated_pos, player, screen_tiles);
    let mut paused = false;
    let mut frame = 0;
    loop {
        clear_background(LIGHTGRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if paused {
            if draw_paused_ui().is_clicked() {
                paused = false;
            }
            next_frame().await;
            continue;
        }
        if is_key_pressed(KeyCode::Down) {
            if game_state.map.move_down() {
                accumulated_pos += DOWN;
            }
        }
        if is_key_pressed(KeyCode::Up) {
            if game_state.map.move_up() {
                accumulated_pos += UP;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            if game_state.map.move_left() {
                accumulated_pos += LEFT;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if game_state.map.move_right() {
                accumulated_pos += RIGHT;
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            let click = Vec2::from(mouse_position());
            let clicked_tile = pixel_to_tile(click.x, click.y, tile_size);
            let tile = game_state.map.get(clicked_tile);
            println!("tile at {:?} is {:?}", clicked_tile, tile);
        }

        let player_tile = game_state.map.get(game_state.map.player);
        if (frame + 1) % 60 == 0 {
            game_state.map.advance();
            if player_tile == Tile::Monster {
                game_state.player_health = 0.0_f32.max(game_state.player_health - 1.0);
            }
        }
        if accumulated_pos == next_door {
            game_state.doors_parts_collected += 1;
            if game_state.doors_parts_collected < REQUIRED_DOORS {
                next_door = calculate_rand_accumulated_pos(accumulated_pos, player, screen_tiles);
            }
        }

        let end_of_map = tile_to_pixel(screen_tiles.x, screen_tiles.y, tile_size);
        draw_rectangle(0.0, 0.0, end_of_map.x, end_of_map.y, COLOR_BACKGROUND);
        draw_map(tile_size, screen_tiles, &game_state.map);
        draw_player(tile_size, player);
        draw_door(tile_size, player, screen_tiles, accumulated_pos, next_door);

        draw_health_ui(game_state.player_health);
        draw_doors_ui(&mut game_state.doors_parts_collected);
        if game_state.player_health <= 0.0 {
            if draw_respawn_ui().is_clicked() {
                game_state = GameState::new(screen_tiles, player);
            }
        }
        if game_state.doors_parts_collected >= 4 {
            if draw_game_won().is_clicked() {
                game_state = GameState::new(screen_tiles, player);
            }
        }

        if is_key_down(KeyCode::F3) {
            draw_text(
                &format!(" FPS: {}", get_fps()),
                0.0,
                screen_height() - FONT_SIZE * 0.5,
                FONT_SIZE,
                BLACK,
            );
        }
        frame = (frame + 1) % 10000;
        next_frame().await
    }
}

fn draw_door(
    tile_size: Pixels2,
    player: Coord2,
    screen_tiles: Coord2,
    accumulated_pos: CoordDiff2,
    next_door: CoordDiff2,
) {
    let door_pos = next_door - accumulated_pos + to_signed(player);
    if door_pos.x >= 0
        && door_pos.x < screen_tiles.x as CoordDiff
        && door_pos.y >= 0
        && door_pos.y < screen_tiles.y as CoordDiff
    {
        let mut pixel = tile_to_pixel(door_pos.x as Coord, door_pos.y as Coord, tile_size);
        pixel += tile_size * 0.25;
        let door_size = tile_size * 0.5;
        draw_rectangle(pixel.x, pixel.y, door_size.x, door_size.y, COLOR_DOOR);
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

fn calculate_rand_accumulated_pos(
    accumulated_pos: CoordDiff2,
    player: Coord2,
    screen_tiles: Coord2,
) -> CoordDiff2 {
    let area = screen_tiles.x * screen_tiles.y;
    let i = (rand() % area) as i32;
    scalar_to_around_accumulated_pos(accumulated_pos, player, screen_tiles, i)
}

fn scalar_to_around_accumulated_pos(
    accumulated_pos: CoordDiff2,
    player: Coord2,
    screen_tiles: Coord2,
    i: i32,
) -> CoordDiff2 {
    let door_pos_unsigned = CoordDiff2::new(i % screen_tiles.x as i32, i / screen_tiles.x as i32);
    let player = to_signed(player);
    let door_pos = door_pos_unsigned - player + accumulated_pos;
    door_pos
}

fn to_signed(pos: Coord2) -> CoordDiff2 {
    CoordDiff2::new(pos.x as CoordDiff, pos.y as CoordDiff)
}

fn draw_map(tile_size: Vec2, screen_tiles: Coord2, map: &Map) {
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
                Tile::Floor => {}
            };
        }
    }
}

fn draw_player(tile_size: Vec2, player: UVec2) {
    let mut pixel = tile_to_pixel(player.x, player.y, tile_size);
    pixel += tile_size * 0.5; // circle position is the center
    draw_circle(pixel.x, pixel.y, 10.0, COLOR_PLAYER);
}

fn draw_health_ui(player_health: f32) {
    let health_unit: Pixels = 20.0;
    let thickness = 1.0;
    draw_rectangle(
        10.0,
        10.0,
        MAX_HEALTH * health_unit + thickness * 2.0,
        health_unit + thickness * 2.0,
        COLOR_UI_DARKER,
    );
    draw_rectangle(
        10.0 + thickness,
        10.0 + thickness,
        player_health * health_unit,
        health_unit,
        COLOR_PLAYER,
    );
}
fn draw_doors_ui(door_parts_collected_mut: &mut i32) {
    let door_parts_collected = *door_parts_collected_mut;
    let door_grid: Pixels = 20.0;
    let door_part: Pixels = 15.0;
    let pad = door_grid - door_part;
    let ui_start_x = screen_width() - door_grid * 2.0 - 10.0;
    let ui_start_y = 10.0;
    let rect = Rect::new(ui_start_x, ui_start_y, door_grid * 2.0, door_grid * 2.0);
    draw_rect(rect, COLOR_UI_LIGHTER);
    draw_rect_lines(rect, 2.0, COLOR_UI_DARKER);
    let draw_door = |x: Pixels, y: Pixels| {
        draw_rectangle(
            ui_start_x + x,
            ui_start_y + y,
            door_part,
            door_part,
            COLOR_DOOR,
        );
    };
    if door_parts_collected > 0 {
        draw_door(pad, pad);
    }
    if door_parts_collected > 1 {
        draw_door(door_grid, pad);
    }
    if door_parts_collected > 2 {
        draw_door(pad, door_grid);
    }
    if door_parts_collected > 3 {
        draw_door(door_grid, door_grid);
    }
    // if is_mouse_button_released(MouseButton::Left)
    //     && rect.contains(Vec2::from(InputMacroquad.mouse_position()))
    // {
    //     *door_parts_collected_mut = (*door_parts_collected_mut + 1) % 5;
    // }
}
fn draw_paused_ui() -> Interaction {
    let text_anchor = Anchor::top_center(screen_width() * 0.5, screen_height() * 0.45);
    let text = TextRect::new("Paused", text_anchor, FONT_SIZE);

    let button_anchor = Anchor::center_below(text.rect, 0.0, 20.0);
    let mut resume = create_button("Resume (Press Space)", button_anchor);
    resume.interact();

    render_window(text.rect.combine_with(resume.rect()));
    text.render_text(COLOR_UI_DARKER);
    resume.render(&STYLE);
    resume.interaction()
}

fn render_window(content: Rect) {
    let pad = 30.0;
    let window = Rect::new(
        content.x - pad,
        content.y - pad,
        content.w + 2.0 * pad,
        content.h + 2.0 * pad,
    );
    draw_rect(window, COLOR_UI_BG);
    draw_rect_lines(window, 2.0, COLOR_UI_DARKER);
}

fn draw_game_won() -> Interaction {
    let text_anchor = Anchor::top_center(screen_width() * 0.5, screen_height() * 0.45);
    let text = TextRect::new("You won!", text_anchor, FONT_SIZE);

    let button_anchor = Anchor::center_below(text.rect, 0.0, 20.0);
    let mut resume = create_button("Restart", button_anchor);
    resume.interact();

    render_window(text.rect.combine_with(resume.rect()));
    text.render_text(COLOR_UI_DARKER);
    resume.render(&STYLE);
    resume.interaction()
}
fn draw_respawn_ui() -> Interaction {
    let text_anchor = Anchor::top_center(screen_width() * 0.5, screen_height() * 0.475);
    let text = TextRect::new("You died", text_anchor, FONT_SIZE);

    let button_anchor = Anchor::center_below(text.rect, 0.0, 20.0);
    let mut retry = create_button("Retry", button_anchor);
    retry.interact();

    render_window(text.rect.combine_with(retry.rect()));
    text.render_text(COLOR_UI_DARKER);
    retry.render(&STYLE);
    retry.interaction()
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

fn pixel_to_tile(x: Pixels, y: Pixels, tile_size: Pixels2) -> Coord2 {
    Coord2::new((x / tile_size.x) as Coord, (y / tile_size.y) as Coord)
}
fn tile_to_pixel(x: Coord, y: Coord, tile_size: Pixels2) -> Pixels2 {
    Pixels2::new(x as Pixels * tile_size.x, y as Pixels * tile_size.y)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_door() {
        let screen_tiles = Coord2::new(4, 10);
        assert_eq!(to_pos(0, screen_tiles), CoordDiff2::new(98, 195));
        assert_eq!(to_pos(1, screen_tiles), CoordDiff2::new(99, 195));
        assert_eq!(
            to_pos((screen_tiles.x * screen_tiles.y) as i32 - 1, screen_tiles),
            CoordDiff2::new(101, 204)
        )
    }

    fn to_pos(i: i32, screen_tiles: UVec2) -> CoordDiff2 {
        scalar_to_around_accumulated_pos(
            CoordDiff2::new(100, 200),
            Coord2::new(2, 5),
            screen_tiles,
            i,
        )
    }
}

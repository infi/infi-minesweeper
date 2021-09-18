mod algo;
mod data;
mod textures;

use std::time::Duration;

use crate::data::{constants, Board, GameState, Tile};
use image::EncodableLayout;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_width: 598,
        window_height: 650,
        window_title: "Minesweeper".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_board(board: &Vec<Vec<Tile>>, atlas: Texture2D) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let x = j as f32;
            let y = i as f32;
            let tile = &board[i][j];

            // The offset of the texture in the atlas.
            let offset = if tile.is_flagged {
                53.0
            } else if tile.is_opened {
                if tile.is_mine {
                    53.0 * 2.0
                } else if tile.number != 0 {
                    (53.0 * 3.0) + 53.0 * tile.number as f32
                } else {
                    0.0
                }
            } else {
                53.0 * 3.0
            };

            draw_texture_ex(
                atlas,
                x * constants::TILE_SIZE + (constants::SPACING * x),
                y * constants::TILE_SIZE + (constants::SPACING * y),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(53.0, 53.0)),
                    source: Some(Rect::new(offset + 1.0, 0.0, 51.0, 53.0)),
                    ..DrawTextureParams::default()
                },
            );
        }
    }
}

fn draw_stats(state: &GameState, font: Font, y_start_coord: f32) {
    let flag_dimensions = measure_text(
        format!("Flags: {}", state.flags_left).as_str(),
        Some(font),
        32,
        1.0,
    );

    draw_text_ex(
        format!("Score: {}", state.score).as_str(),
        10.0,
        y_start_coord,
        TextParams {
            font: font,
            font_size: 32,
            color: WHITE,
            ..TextParams::default()
        },
    );
    draw_text_ex(
        format!("Flags: {}", state.flags_left).as_str(),
        window_conf().window_width as f32 - flag_dimensions.width - 10.0,
        y_start_coord,
        TextParams {
            font: font,
            font_size: 32,
            color: if state.flags_left >= 0 { WHITE } else { RED },
            ..TextParams::default()
        },
    );
}

fn draw_message(state: &GameState, font: Font) {
    draw_rectangle(
        10.0,
        10.0,
        window_conf().window_width as f32 - 20.0,
        100.0,
        BLACK,
    );
    draw_text_ex(
        if state.lost {
            "Oh no!"
        } else if state.won {
            "Congratulations!"
        } else {
            "Note"
        },
        20.0,
        45.0,
        TextParams {
            font: font,
            font_size: 30,
            color: if state.won { YELLOW } else { GRAY },
            ..TextParams::default()
        },
    );
    draw_text_ex(
        if state.lost {
            "You just lost the game. You also lost minesweeper."
        } else if state.won {
            "You won the game!"
        } else {
            state.message.as_str()
        },
        20.0,
        70.0,
        TextParams {
            font: font,
            font_size: 20,
            color: WHITE,
            ..TextParams::default()
        },
    );

    if state.lost || state.won {
        draw_text_ex(
            "Press R to try again",
            20.0,
            92.5,
            TextParams {
                font: font,
                font_size: 20,
                color: GRAY,
                ..TextParams::default()
            },
        );
    }
}

fn calculate_mouse_tile() -> (usize, usize) {
    let mouse_x = mouse_position().0;
    let mouse_y = mouse_position().1;

    // Calculate on which tile the mouse is.
    let tile_x = (mouse_x / (constants::TILE_SIZE + constants::SPACING)) as usize;
    let tile_y = (mouse_y / (constants::TILE_SIZE + constants::SPACING)) as usize;

    (tile_x, tile_y)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new(constants::WIDTH, constants::HEIGHT);
    board.place_random_mines(constants::MINES_PER_BOARD);
    board.recalculate_adjacent_mines();

    let mut state = GameState::new(constants::MINES_PER_BOARD);

    let font =
        load_ttf_font_from_bytes(include_bytes!("assets/fonts/InfiMinesweeper.ttf")).unwrap();

    let stitched = textures::stitch();
    let atlas = Texture2D::from_rgba8(stitched.0, stitched.1, stitched.2.as_bytes());

    let scoreboard_start = (constants::TILE_SIZE * constants::HEIGHT as f32)
        + (constants::SPACING * constants::HEIGHT as f32)
        + 30.0;

    loop {
        clear_background(BLACK);

        draw_board(&board.itself, atlas);

        draw_stats(&state, font, scoreboard_start);

        if state.message.len() > 0 || state.lost || state.won {
            draw_message(&state, font);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (tile_x, tile_y) = calculate_mouse_tile();

            if !(tile_x >= constants::WIDTH || tile_y >= constants::HEIGHT) {
                let tile = &mut board.itself[tile_y][tile_x];
                if tile.is_mine {
                    tile.is_opened = true;
                    state.lost = true;
                } else if tile.number > 0 {
                    tile.is_opened = true;
                } else {
                    algo::flood_fill(&mut board, tile_x, tile_y);
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (tile_x, tile_y) = calculate_mouse_tile();

            if !(tile_x >= constants::WIDTH || tile_y >= constants::HEIGHT) {
                if board.itself[tile_y as usize][tile_x as usize].is_flagged {
                    board.itself[tile_y as usize][tile_x as usize].is_flagged = false;
                    state.take_flag();
                } else {
                    board.itself[tile_y as usize][tile_x as usize].is_flagged = true;
                    state.put_flag();

                    let mut flagged_mines = 0;

                    for row in &board.itself {
                        for tile in row {
                            if tile.is_mine && tile.is_flagged {
                                flagged_mines += 1;
                            }
                        }
                    }

                    if flagged_mines == constants::MINES_PER_BOARD {
                        state.won = true;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            // mark all tiles as not opened and not flagged
            for i in 0..board.itself.len() {
                for j in 0..board.itself[i].len() {
                    board.itself[i][j].is_opened = false;
                    board.itself[i][j].is_flagged = false;
                }
            }
            // reset the game
            state = GameState::new(constants::MINES_PER_BOARD);
            board.place_random_mines(constants::MINES_PER_BOARD);
            board.recalculate_adjacent_mines();
        }

        if is_key_pressed(KeyCode::Q) {
            // quit the game
            break;
        }

        next_frame().await;
        std::thread::sleep(Duration::from_millis(10));
    }
}

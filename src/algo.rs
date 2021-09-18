use crate::data::{constants, Board};

// Messy. Refactor.
// Flood fill algorithm.
pub fn flood_fill(board: &mut Board, x: usize, y: usize) {
    let mut tile = &mut board.itself[y][x];

    if tile.is_opened || tile.is_flagged || tile.is_mine {
        return;
    }

    tile.is_opened = true;

    if tile.number > 0 {
        if y > 0 {
            let tile_at = &mut board.itself[y - 1][x];
            if tile_at.number > 0 && !tile_at.is_mine {
                tile_at.is_opened = true;
            }
        }
        if y < constants::HEIGHT - 1 {
            let tile_at = &mut board.itself[y + 1][x];
            if tile_at.number > 0 && !tile_at.is_mine {
                tile_at.is_opened = true;
            }
        }
        if x > 0 {
            let tile_at = &mut board.itself[y][x - 1];
            if tile_at.number > 0 && !tile_at.is_mine {
                tile_at.is_opened = true;
            }
        }
        if x < constants::WIDTH - 1 {
            let tile_at = &mut board.itself[y][x + 1];
            if tile_at.number > 0 && !tile_at.is_mine {
                tile_at.is_opened = true;
            }
        }

        return;
    }

    if y > 0 {
        flood_fill(board, x, y - 1);
    }
    if y < constants::HEIGHT - 1 {
        flood_fill(board, x, y + 1);
    }
    if x > 0 {
        flood_fill(board, x - 1, y);
    }
    if x < constants::WIDTH - 1 {
        flood_fill(board, x + 1, y);
    }
}

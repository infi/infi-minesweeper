use rand::Rng;

pub struct Tile {
    pub is_mine: bool,
    pub is_opened: bool,
    pub is_flagged: bool,
    pub number: i32,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            is_mine: false,
            is_opened: false,
            is_flagged: false,
            number: 0,
        }
    }
}

pub struct Board {
    pub itself: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut field = Vec::<Vec<Tile>>::new();
        for _i in 0..height {
            let mut row = Vec::<Tile>::new();
            for _j in 0..width {
                row.push(Tile::new());
            }
            field.push(row);
        }

        Board {
            itself: field,
            width: width,
            height: height,
        }
    }

    pub fn place_random_mines(&mut self, count: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.itself[y][x].is_mine = false;
            }
        }

        for _ in 0..count {
            let x = rand::thread_rng().gen_range(0..self.width);
            let y = rand::thread_rng().gen_range(0..self.height);
            if self.itself[x][y].is_mine {
                continue;
            }
            self.itself[x][y].is_mine = true;
        }
    }

    pub fn recalculate_adjacent_mines(&mut self) {
        let board_width = self.itself.len();
        let board_height = self.itself[0].len();

        for x in 0..board_width {
            for y in 0..board_height {
                let mut count = 0;
                if x > 0 {
                    if self.itself[x - 1][y].is_mine {
                        count += 1;
                    }
                }
                if x < board_width - 1 {
                    if self.itself[x + 1][y].is_mine {
                        count += 1;
                    }
                }
                if y > 0 {
                    if self.itself[x][y - 1].is_mine {
                        count += 1;
                    }
                }
                if y < board_height - 1 {
                    if self.itself[x][y + 1].is_mine {
                        count += 1;
                    }
                }
                if x > 0 && y > 0 {
                    if self.itself[x - 1][y - 1].is_mine {
                        count += 1;
                    }
                }
                if x < board_width - 1 && y > 0 {
                    if self.itself[x + 1][y - 1].is_mine {
                        count += 1;
                    }
                }
                if x > 0 && y < board_height - 1 {
                    if self.itself[x - 1][y + 1].is_mine {
                        count += 1;
                    }
                }
                if x < board_width - 1 && y < board_height - 1 {
                    if self.itself[x + 1][y + 1].is_mine {
                        count += 1;
                    }
                }
                self.itself[x][y].number = count;
            }
        }
    }
}

pub struct GameState {
    pub score: i32,
    pub flags_left: i32,
    pub won: bool,
    pub lost: bool,
    pub message: String,
}

impl GameState {
    pub fn new(mines: usize) -> GameState {
        GameState {
            score: 0,
            flags_left: mines as i32,
            won: false,
            lost: false,
            message: String::new(),
        }
    }

    pub fn put_flag(&mut self) {
        self.flags_left -= 1;
    }

    pub fn take_flag(&mut self) {
        self.flags_left += 1;
    }
}

pub mod constants {
    pub static TILE_SIZE: f32 = 53.0;
    pub static SPACING: f32 = 7.5;
    pub static WIDTH: usize = 10;
    pub static HEIGHT: usize = 10;
    pub static MINES_PER_BOARD: usize = 15;
}

use std::{thread, time};

const ALIVE_STATE: bool = true;
const DEAD_STATE: bool = false;
const ALIVE_CHAR: char = '#';
const DEAD_CHAR: char = ' ';

#[derive(Clone, Debug)]
pub struct GameBoard {
    board_state: Vec<Vec<bool>>,
    board_width: usize,
    board_height: usize,
    current_population: u32,
    generations: u64,
    time_per_tick_ms: u64,
}

impl GameBoard {
    pub fn new(initial_state_raw: String, time_per_tick_ms: u64) -> Self {
        let initial_game_state = GameBoard::create_initial_game_board(initial_state_raw);
        Self {
            board_state: initial_game_state.0,
            board_width: initial_game_state.1 as usize,
            board_height: initial_game_state.2 as usize,
            current_population: initial_game_state.3,
            generations: 0,
            time_per_tick_ms: time_per_tick_ms,
        }
    }

    pub fn play_game(&mut self) {
        loop {
            self.do_step();
            self.render();
            thread::sleep(time::Duration::from_millis(self.time_per_tick_ms));
        }
    }

    /// Steps the game forward by one more generation.
    /// 
    /// Each alive cell with less than two or greater than three alive neighbours dies. Each dead
    /// cell with exactly three alive neighbours is born.
    pub fn do_step(&mut self) {
        let mut new_state: Vec<Vec<bool>> =
            vec![vec![false; self.board_width as usize]; self.board_height as usize];
        for row_index in 0..self.board_height {
            for col_index in 0..self.board_width {
                let current_state = self.get_state(row_index, col_index);
                let alive_neighbours = self.count_alive_neighbours(row_index, col_index);
                if current_state == ALIVE_STATE {
                    if (2..4).contains(&alive_neighbours) {
                        new_state[row_index][col_index] = ALIVE_STATE;
                    } else {
                        self.current_population -= 1;
                    }
                } else { // current_state == DEAD_STATE
                    if alive_neighbours == 3 {
                        new_state[row_index][col_index] = ALIVE_STATE;
                        self.current_population += 1;
                    }
                }
            }
        }
        self.board_state = new_state;
        self.generations += 1;
    }

    fn get_state(&self, row_index: usize, col_index: usize) -> bool {
        return self.board_state[row_index][col_index];
    }

    fn count_alive_neighbours(&self, row_index: usize, col_index: usize) -> u32 {
        let mut dead_neighbours: u32 = 0;
        // Top left
        if col_index == 0 || row_index == 0 || self.board_state[row_index-1][col_index-1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Top
        if row_index == 0 || self.board_state[row_index-1][col_index] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Top right
        if col_index == self.board_width-1 || row_index == 0 || self.board_state[row_index-1][col_index+1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Left
        if col_index == 0 || self.board_state[row_index][col_index-1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Right
        if col_index == self.board_width-1 || self.board_state[row_index][col_index+1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Bottom left
        if col_index == 0 || row_index == self.board_height-1 || self.board_state[row_index+1][col_index-1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Bottom
        if row_index == self.board_height-1 || self.board_state[row_index+1][col_index] == DEAD_STATE {
            dead_neighbours += 1;
        }
        // Bottom right
        if col_index == self.board_width-1 || row_index == self.board_height-1 || self.board_state[row_index+1][col_index+1] == DEAD_STATE {
            dead_neighbours += 1;
        }
        return 8 - dead_neighbours;
    }

    // Prints out the current state of the game in the console window.
    pub fn render(&self) {
        // Clear console window first
        print!("{}[2J", 27 as char);
        print!("{}[0;0H", 27 as char);
        // Add first line as border
        let mut game_board_state = String::new();
        game_board_state.push_str("Game of Life\n");
        game_board_state.push_str(&(0..self.board_width + 2).map(|_| "*").collect::<String>());
        game_board_state.push_str("\n");
        // Add grid lines
        for row_index in 0..self.board_height {
            let mut row = String::new();
            row.push('*');
            for col_index in 0..self.board_width {
                let state = self.board_state[row_index as usize][col_index as usize];
                match state {
                    ALIVE_STATE => row.push(ALIVE_CHAR),
                    DEAD_STATE => row.push(DEAD_CHAR),
                }
            }
            row.push_str("*\n");
            game_board_state.push_str(&row);
        }
        // Add bottom row as border
        game_board_state.push_str(&(0..self.board_width + 2).map(|_| "*").collect::<String>());
        game_board_state.push_str("\n");
        // Add some stats
        game_board_state.push_str(&format!("Current generation: {}\n", self.generations));
        game_board_state.push_str(&format!("Current population: {}\n", self.current_population));
        game_board_state.push_str(&format!("Time per tick (ms): {}\n", self.time_per_tick_ms));
        print!("{}", game_board_state);
    }

    /// Produces the initial game board from the raw input. Panics if the game board input contains
    /// incorrect characters or has incorrect formatting.
    fn create_initial_game_board(initial_state_raw: String) -> (Vec<Vec<bool>>, u32, u32, u32) {
        let mut board_state: Vec<Vec<bool>> = vec![];
        let mut initial_population = 0;
        // Read first line to get grid width and height
        let mut initial_state_lines = initial_state_raw.lines();
        let first = initial_state_lines.next();
        let dimensions = match first {
            Some(v) => v
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
            None => panic!("ERROR - could not read first line for grid dimensions."),
        };
        // Check if we've been given a bad number of dimensions
        if dimensions.len() != 2 {
            panic!("ERROR - bad number of dimensions given.");
        }
        let board_width: u32 = dimensions[0];
        let board_height: u32 = dimensions[1];
        // Now we read the rest of the grid
        let mut rows_added = 0;
        for line in initial_state_lines {
            let line = line.trim();
            // Ignore empty lines
            if line.is_empty() {
                continue;
            }
            // Check if we try to add too many rows from input file
            if rows_added == board_height {
                panic!("ERROR - input file has too many rows.");
            }
            let mut row: Vec<bool> = vec![];
            // Split row values and parse to integer value
            let mut row_state = vec![];
            for c in line.split(",") {
                // Ignore additional spaces at end of file
                if c.is_empty() {
                    continue;
                }
                // Check if the current row is too long
                if row_state.len() == board_width as usize {
                    panic!(format!(
                        "ERROR - input file has bad row length for row {}",
                        rows_added
                    ));
                }
                let result = c.parse::<u32>().unwrap();
                row_state.push(result);
            }
            // Generate the row to be added to the board state
            for state in row_state {
                match state {
                    1 => {
                        row.push(true);
                        initial_population += 1;
                    },
                    0 => row.push(false),
                    _ => panic!("ERROR - unexpected state value observed."),
                }
            }
            board_state.push(row);
            rows_added += 1;
        }
        return (board_state, board_width, board_height, initial_population);
    }
}

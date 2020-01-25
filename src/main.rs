use std::env;
use std::fs;

mod game;

fn main() {
    // Read input file to string
    let args: Vec<String> = env::args().collect();   
    if args.len() != 3 {
        panic!(format!("Usage: game_of_life <input_file> <time per tick (ms)"));
    }
    let initial_input_raw = fs::read_to_string(args[1].clone()).unwrap();
    let time_per_tick_ms = args[2].parse::<u64>().unwrap();
    let mut game_state = game::GameBoard::new(initial_input_raw, time_per_tick_ms);
    game_state.play_game();
}

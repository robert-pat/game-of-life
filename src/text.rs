use crate::game;
use crate::game::GameAction;
use crate::{save_load, GAME_X, GAME_Y};
use lazy_static::lazy_static;
use regex::Regex;
pub(crate) fn text() -> ! {
    println!("Welcome to the Game of Life!");
    let start = initialize_board();
    run_command_line(start);
}
fn initialize_board() -> game::GameBoardOld {
    let std_in = std::io::stdin();
    println!("Start (m)anually or (l)oad from file? (\"Enter\" to skip)");

    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Couldn't read stdIn");

    match input.trim() {
        "l" => {
            let p = get_file_path();
            save_load::load_board_from_file(p.trim())
        }
        "m" => {
            let mut new_board = game::GameBoardOld::new(GAME_X, GAME_Y);
            new_board.set_cells(get_coordinates(&std_in), game::CellState::Alive);
            new_board
        }
        _ => game::GameBoardOld::new(GAME_X, GAME_Y),
    }
}
fn run_command_line(mut board: game::GameBoardOld) -> ! {
    let std_in = std::io::stdin();

    loop {
        match get_user_game_action(&std_in) {
            GameAction::Step => board = game::run_iterations(&board, get_user_number(&std_in)),
            GameAction::GrowCell => prompt_user_to_change_cells(&mut board, game::CellState::Alive),
            GameAction::KillCell => prompt_user_to_change_cells(&mut board, game::CellState::Dead),

            GameAction::Play => {
                // "Play" the simulation until stopped, or everything dies
                println!("The sim will run until all cells are dead, use ^C to stop.");
                let mut count = 0;
                while board.has_alive_cells() {
                    display_next_iteration(&board, count > 0, count);
                    board = game::run_iterations(&board, 1);
                    count += 1;
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                println!("All Cells died:\n{}", board);
                std::process::exit(0);
            }
            GameAction::Save => user_save_board(&board),
            GameAction::PrintBoard => {
                println!("{}", board)
            }
            GameAction::Quit => std::process::exit(0),
            GameAction::Paused => println!("Game is Paused!"),
            GameAction::Failed => eprintln!("Failed to parse, sorry!"),
        }
    }
}

/// Saves the board to the specified file
fn user_save_board(board: &game::GameBoardOld) {
    println!("Where would you like to save the board?");
    let p = get_file_path();
    save_load::save_board(p.trim(), board);
}

/// Prompts a user to pick cells to change on the board & changes them to the specified Status
/// Allows for both file reading and manually typing in cells
pub(crate) fn prompt_user_to_change_cells(board: &mut game::GameBoardOld, status: game::CellState) {
    let std_in = std::io::stdin();
    println!("(t)ype in coordinates or (r)ead from a file?");

    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    match input.trim() {
        "t" => board.set_cells(get_coordinates(&std_in), status),
        "r" => {
            let p = get_file_path();
            board.set_cells(save_load::read_coords_from_file(p.trim()), status);
        }
        _ => eprintln!("Error, No Cells Changed."),
    }
}
/// Prints the board to the terminal, replacing previous text if replace_prev is true
fn display_next_iteration(board: &game::GameBoardOld, replace_prev: bool, gen: i32) {
    if replace_prev {
        for _ in 0..=board.y_max {
            print!("{}", ansi_escapes::CursorPrevLine);
        }
    }
    println!("Generation: {gen}\n{board}");
}

/// Prompts the user for any number of coordinates
pub fn get_coordinates(std_in: &std::io::Stdin) -> Vec<(usize, usize)> {
    let mut input: String = String::new();

    println!("Enter coordinate(s): x,y x,y x,y ...");
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    parse_to_coordinates(input)
}

/// Parses a given string into a Vec of coordinates, uses regex to match number,number patterns
/// This function handles parsing for get_user_coordinate_vec() & read_file_coordinates()
pub(crate) fn parse_to_coordinates(input: String) -> Vec<(usize, usize)> {
    let mut cells: Vec<(usize, usize)> = Vec::new();

    // from the https://docs.rs/regex/latest/regex/ page
    lazy_static! {
        pub static ref FILTER: Regex = Regex::new(r"([\d]+,[\d]+)+").unwrap();
    }

    for c in FILTER.captures_iter(&input) {
        let pair: Vec<_> = c.get(0).unwrap().as_str().split(',').collect();
        cells.push((pair[0].parse().unwrap(), pair[1].parse().unwrap()));
    }

    cells
}

/// Prompts the user for a single number
pub(crate) fn get_user_number(std_in: &std::io::Stdin) -> usize {
    let mut input: String = String::new();
    println!("Please enter a number:");
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    return match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Couldn't parse number");
            0
        }
    };
}
/// Reads a line from the console & parses it into a Action
pub fn get_user_game_action(std_in: &std::io::Stdin) -> GameAction {
    println!("Pick an option:");
    println!("(s)imulate, (g)row/(k)ill cells, (p)rint the board, (l)et the sim. run, sa(v)e the board, (q)uit/(c)ancel");
    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    return match input.trim() {
        "s" => GameAction::Step,
        "p" => GameAction::PrintBoard,
        "g" => GameAction::GrowCell,
        "k" => GameAction::KillCell,
        "q" | "c" => GameAction::Quit,
        "l" => GameAction::Play,
        "v" => GameAction::Save,
        _ => GameAction::Failed,
    };
}

pub(crate) fn get_file_path() -> String {
    let mut s = String::new();
    println!("Please enter a file path:");
    match std::io::stdin().read_line(&mut s) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading stdIn: {e}");
            return "default.txt".to_string();
        }
    };
    s
}

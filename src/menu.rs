use crate::{user_io, GAME_X, GAME_Y};
use crate::{game};
use crate::user_io::GameAction;

pub fn setup_initial_board() -> game::GameBoard {
    let std_in = std::io::stdin();

    println!("How would you like to start?");
    println!("(m)anually or (l)oaded from a file? Pressing \"Enter\" starts normally.");

    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Couldn't read std in");

    return match input.trim() {
        "l" => {
            println!("File name?");
            input.clear();
            std_in.read_line(&mut input).expect("Failed reading stdIn");

            user_io::load_board_from_file(input.trim().to_string())
        },
        _ => {
            let mut new_board = game::GameBoard::new(GAME_X, GAME_Y);
            new_board.set_cells(user_io::get_user_coordinate_vec(&std_in), game::CellStatus::Alive);
            new_board
        }
    }
}

pub fn command_line_control_loop(mut board: game::GameBoard){
    let std_in = std::io::stdin();

    loop{
        match user_io::get_user_game_action(&std_in){
            GameAction::Simulation => board = game::run_iterations(&board, user_io::get_user_number(&std_in)),
            GameAction::GrowCell => prompt_user_to_change_cells(&mut board, game::CellStatus::Alive, &std_in),
            GameAction::KillCell => prompt_user_to_change_cells(&mut board, game::CellStatus::Dead, &std_in),

            GameAction::Play => { // "Play" the simulation until stopped, or everything dies
                println!("The sim will run until all cells are dead, use ^C to stop.");
                let mut count = 0;
                while board.has_alive_cells(){
                    display_next_iteration(&board,count > 0, count);
                    board = game::run_iterations(&board, 1);
                    count += 1;
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
                println!("All Cells died:\n{}", board);
                break;
            },
            GameAction::Save => prompt_user_to_save_board(&board, &std_in),
            GameAction::PrintBoard => {println!("{}", board)},
            GameAction::Quit => break,
            GameAction::Failed => eprintln!("Failed to parse, sorry!")
        }
    }
}

/// Saves the board to the specified file
pub fn prompt_user_to_save_board(board: &game::GameBoard, std_in: &std::io::Stdin){
    println!("Where would you like to save the board?");
    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    user_io::save_board_to_file(input.trim(), board);
}

/// Prompts a user to pick cells to change on the board & changes them to the specified Status
/// Allows for both file reading and manually typing in cells
pub fn prompt_user_to_change_cells(board: &mut game::GameBoard, status: game::CellStatus, std_in: &std::io::Stdin){
    println!("(t)ype in coordinates or (r)ead from a file?");

    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    match input.trim() {
        "t" => board.set_cells(user_io::get_user_coordinate_vec(std_in), status),
        "r" => {
            println!("Enter the file name:");
            input.clear();
            std_in.read_line(&mut input).expect("Failed reading stdIn");
            board.set_cells(user_io::read_file_coordinates(input.trim()), status);
        },
        _ => eprintln!("Error, No Cells Changed.")
    }
}

/// Prints the board to the terminal, replacing previous text if replace_prev is true
pub fn display_next_iteration(board: &game::GameBoard, replace_prev: bool, gen: i32){
    if replace_prev {
        for _ in 0..=board.y_max{
            print!("{}", ansi_escapes::CursorPrevLine);
        }
    }
    println!("Generation: {gen}\n{board}");
}
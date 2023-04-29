#![allow(unused)]
mod game;
mod cust_io;
mod debug;
mod menu;

use ctrlc;

const GAME_X: usize = 30;
const GAME_Y: usize = 30;

const ALIVE_STATUS_CHARACTER: char = '☑';
const DEAD_STATUS_CHARACTER: char = '☒';

fn main() {
    ctrlc::set_handler(|| {std::process::exit(0);});
    if false {menu::debug_main(); return;} // Whether the program is in debug mode

    let mut board = game::Board::new(GAME_X, GAME_Y); // Create the program's board
    let std_in = std::io::stdin();
    let mut std_out = std::io::stdout();

    // Get the initial conditions for the simulation
    board = menu::initial_game_setup(GAME_X, GAME_Y, &std_in);

    loop{
        println!("Options: (s)im. n iterations, (g)row cells, (k)ill cells, (p)rint the board, (l)et the sim. run, sa(v)e the board, (q)uit/(c)ancel");
        let choice = cust_io::get_user_choice(&std_in);

        match choice{
            cust_io::Action::Simulation => board = game::run_iterations(&board, cust_io::get_user_number(&std_in)),

            cust_io::Action::GrowCell => {
                game::set_cells(&mut board, menu::promt_user_to_change_cells(&std_in), game::Status::Alive);
            },

            cust_io::Action::KillCell => {
                game::set_cells(&mut board, menu::promt_user_to_change_cells(&std_in), game::Status::Dead);
            },

            cust_io::Action::Play => {
                println!("The sim will run forever, use ^C to stop.");
                let mut count: usize = 0;
                loop{
                    menu::display_next_iteratrion(&board, &mut std_out, {count > 0}, count);
                    board = game::run_iterations(&board, 1);
                    count += 1;
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
            },

            cust_io::Action::Save => {
                println!("Where would you like to save the board?");
                let mut input: String = String::new();
                std_in.read_line(&mut input);

                cust_io::save_board_to_file(input.trim(), board);
                break;
            },

            cust_io::Action::PrintBoard => {println!("{}", board)}, 
            cust_io::Action::Cancel => break,
            cust_io::Action::Failed => eprintln!("Failed to parse, sorry!")
        }
    }
    println!("Ending program");
}

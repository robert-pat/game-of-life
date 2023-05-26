mod game;
mod user_io;
mod debug;
mod menu;

use ctrlc;

const GAME_X: usize = 30;
const GAME_Y: usize = 30;

const ALIVE_STATUS_CHARACTER: char = '✓'; // ☑
const DEAD_STATUS_CHARACTER: char = '✗'; // ☒

enum ProgramMode{
    CommandLine,
    Debug,
    GUI
}

fn get_app_mode() -> ProgramMode{
    let mut args = std::env::args();
    if !args.len() > 1{
        return ProgramMode::CommandLine;
    }
    match args.nth(1).unwrap_or(String::new()).as_str(){
        "-d" => ProgramMode::Debug,
        "-g" => ProgramMode::GUI,
        "-c" | "-s" | "-n" => ProgramMode::CommandLine,
        _ => ProgramMode::CommandLine
    }
}

fn main() {
    // Set an exit handler, so the panic error doesn't show up when the program is quit
    match ctrlc::set_handler(|| {std::process::exit(0);}){
        Ok(_) => {},
        Err(_) => eprintln!("Failed to set process exit handler")
    }

    let mode: ProgramMode = get_app_mode();
    match mode{
        ProgramMode::Debug => {
            println!("Running Tests");
            match menu::run_tests(){
                Ok(_) => println!("Tests Passed"),
                Err(e) => eprintln!("Test Failed!, {:?}", e)
            }

            println!("Debug Mode Running..");
            menu::run_debug();
        },
        ProgramMode::CommandLine => {
            let board = menu::setup_initial_board();
            menu::command_line_control_loop(board);
        }
        ProgramMode::GUI => todo!()
    }
}



mod game;
mod user_io;
mod debug;
mod menu;

const GAME_X: usize = 30;
const GAME_Y: usize = 30;

const ALIVE_STATUS_CHARACTER: char = '✓'; // ☑
const DEAD_STATUS_CHARACTER: char = '✗'; // ☒

enum ProgramMode{
    CommandLine,
    Testing,
    Gui
}

fn get_app_mode() -> ProgramMode{
    let mut args = std::env::args();
    if args.len() < 1{
        return ProgramMode::CommandLine;
    }
    match args.nth(1).unwrap_or(String::new()).as_str(){
        "-d" => ProgramMode::Testing,
        "-g" => ProgramMode::Gui,
        _ => ProgramMode::CommandLine
    }
}

fn main() {
    // Set an exit handler, so the panic error doesn't show up when the program is quit
    match ctrlc::set_handler(|| {std::process::exit(0);}){
        Ok(_) => {},
        Err(_) => eprintln!("Failed to set process exit handler")
    }

    match get_app_mode(){
        ProgramMode::Testing => {
            println!("{:?}", std::env::args());
            println!("Test Mode Running-> Line Rewriting, Print all Raw Neighbors");
            debug::line_rewriting_demo();
            debug::print_all_raw_neighbors();
        },
        ProgramMode::CommandLine => {
            let board = menu::setup_initial_board();
            menu::command_line_control_loop(board);
        }
        ProgramMode::Gui => todo!()
    }
}

use winit::dpi::PhysicalSize;

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
    Gui,
}
const DEFAULT_MODE: ProgramMode = ProgramMode::Gui;
fn get_app_mode() -> ProgramMode{
    // TODO: rework this
    let args = std::env::args();
    if args.len() < 1 { return ProgramMode::CommandLine; }
    let mut mode = ProgramMode::CommandLine;
    args.for_each(|arg| {
        match arg.as_str(){
            "-g" => { mode = ProgramMode::Gui; },
            _ => {}
        }
    });
    mode
}
fn main() {
    // Set an exit handler, so the panic error doesn't show up when the program is quit
    ctrlc::set_handler(| | {std::process::exit(0); }).expect("Failed to set Handler!");

    match get_app_mode(){
        ProgramMode::CommandLine => {
            let board = menu::setup_initial_board();
            menu::run_command_line(board);
        }
        ProgramMode::Gui => {
            let size = PhysicalSize::new((GAME_X * 6) as u32, (GAME_Y * 6) as u32);
            let (pixels, window, event_loop) = menu::gui_init(size);
            menu::run_gui(event_loop, window, pixels);
        },
    }
}

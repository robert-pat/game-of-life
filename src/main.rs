use graphics::GUIGameState;
use winit::dpi::PhysicalSize;

mod game;
mod graphics;
mod save_load;
mod test;
mod text;

const GAME_X: usize = 120;
const GAME_Y: usize = 80;
const ALIVE_STATUS_CHARACTER: char = '✓'; // ☑
const DEAD_STATUS_CHARACTER: char = '✗'; // ☒
enum ProgramMode {
    CommandLine,
    Gui,
}
fn get_app_mode() -> ProgramMode {
    ProgramMode::Gui
}
fn main() {
    // Set an exit handler, so the panic error doesn't show up when the program is quit
    ctrlc::set_handler(|| {
        std::process::exit(0);
    })
    .expect("Failed to set Handler!");

    match get_app_mode() {
        ProgramMode::CommandLine => {
            let board = text::initialize_board();
            text::run_command_line(board);
        }
        ProgramMode::Gui => graphics::gui(),
    }
}

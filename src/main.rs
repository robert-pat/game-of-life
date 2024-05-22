mod game;
mod graphics;
mod save_load;
mod test;
mod text;

const GAME_X: usize = 120;
const GAME_Y: usize = 80;
const ALIVE_STATUS_CHARACTER: char = '✓'; // ☑
const DEAD_STATUS_CHARACTER: char = '✗'; // ☒

#[allow(unused)]
enum ProgramMode {
    CommandLine,
    Gui,
}

fn get_app_mode() -> ProgramMode {
    ProgramMode::Gui // TODO
}

fn main() {
    // Set an exit handler, so the panic error doesn't show up when the program is quit
    ctrlc::set_handler(|| {
        std::process::exit(0); // TODO: this isn't the ideal way of exiting
    })
    .expect("Failed to set Handler!");

    if std::env::args().nth(1).is_some() {
        graphics::incrementing_render();
    }
    match get_app_mode() {
        ProgramMode::CommandLine => text::text(),
        ProgramMode::Gui => graphics::gui(Some(save_load::load_game("board.txt").unwrap())),
    }
}

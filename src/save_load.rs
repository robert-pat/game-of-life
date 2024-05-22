use crate::game::CellState;
use crate::{game, text, ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER, GAME_X, GAME_Y};
use core::str;

//noinspection SpellCheckingInspection
pub fn read_coords_from_file(path: &str) -> Vec<(usize, usize)> {
    match std::fs::read_to_string(path) {
        Ok(contents) => text::parse_to_coordinates(contents),
        Err(e) => {
            eprintln!("Failed to read coordinates from file: {e}!");
            Vec::new()
        }
    }
}
/// Writes the given game board to the specified file.
/// This will replace the file if it already exists
pub fn save_board(path: &str, board: &game::GameBoardOld) {
    let mut contents: String = String::new();

    for row in &board.space {
        for cell in row {
            contents.push((*cell).into());
        }
        contents.push('\n');
    }
    // a newline in appended to the end of each row, even the last one (this removes it)
    contents.pop();
    match std::fs::write(path, contents) {
        Ok(_) => println!("Saved Successfully!"),
        Err(_) => eprintln!("Error Saving Board"),
    };
}

/// Loads a game board from a file.
/// If the file is improperly formatted, it will return an empty board.
/// Failing to load the board is logged to std err
pub fn load_board_from_file(path: &str) -> game::GameBoardOld {
    let mut constructed_board: Vec<Vec<CellState>> = Vec::new();
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to load board from file");
            return game::GameBoardOld::new(GAME_X, GAME_Y);
        }
    };

    for row in contents.split('\n') {
        let mut constructed_row: Vec<CellState> = Vec::new();
        for s in row.chars() {
            constructed_row.push(match s {
                ALIVE_STATUS_CHARACTER => CellState::Alive,
                DEAD_STATUS_CHARACTER => CellState::Dead,
                _ => {
                    eprintln!("Error parsing char from file: [{}]", s);
                    continue;
                } //Don't push anything on error
            });
        }
        constructed_board.push(constructed_row);
    }

    game::GameBoardOld {
        x_max: constructed_board[0].len(),
        y_max: constructed_board.len(),
        space: constructed_board, // last to avoid borrowing after move
    }
}

pub fn load_board_from_file_new(path: &str) -> game::Game {
    let old_board = load_board_from_file(path);
    let mut new = game::Game::new(old_board.x_max, old_board.y_max);
    assert!(new.clone_from_old(&old_board).is_ok());
    new
}

/// Converts a raw text board from conwaylife.com into internal game representation
#[allow(unused)]
pub fn convert_wiki_to_board(path: &str) -> game::GameBoardOld {
    // Load the text from the file, comments are marked w/ "!"
    let mut file = std::fs::read_to_string(path).unwrap();

    let mut x_max: usize = 0;
    for row in file.split('\n').filter(|r| !r.contains('!')) {
        let mut count = 0; // Find the largest row to set the board size
        row.chars().for_each(|c| {
            if c == '.' || c == 'O' {
                count += 1;
            }
        });
        if count > x_max {
            x_max = count;
        }
    }

    let mut board = game::GameBoardOld::new(x_max, file.lines().count());
    for (y, row) in file.split('\n').filter(|r| !r.contains('!')).enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '.' => board.set(x, y, CellState::Dead),
                'O' => board.set(x, y, CellState::Alive),
                _ => {}
            }
        }
    }
    board
}
/// Overwrites a conwaylife.com text board into a game save file
#[allow(unused)]
pub fn create_save_from_wiki(path: &str) {
    let board = convert_wiki_to_board(path); // Load a board from the file
    save_board(path, &board); // Write the board to the original file
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum SaveLoadError {
    StringWrite,
    FileWrite,
    FileOpen,
    EmptyFile,
}
impl From<std::fmt::Error> for SaveLoadError {
    fn from(_: std::fmt::Error) -> Self {
        SaveLoadError::StringWrite
    }
}
impl From<std::io::Error> for SaveLoadError {
    fn from(_: std::io::Error) -> Self {
        SaveLoadError::FileWrite
    }
}
#[allow(unused)]
pub(crate) fn save_game(game: &game::Game, path: &str) -> Result<(), SaveLoadError> {
    use std::fmt::Write as fmtWrite;
    use std::io::Write as ioWrite;

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut s = String::with_capacity(game.y_max * game.x_max);

    for row in game.rows() {
        for cell in row {
            write!(s, "{}", *cell)?;
        }
        writeln!(s)?;
    }
    s.pop(); // remove last new line
    write!(f, "{s}")?;
    Ok(())
}
#[allow(unused)]
pub(crate) fn load_game(path: &str) -> Result<game::Game, SaveLoadError> {
    let mut f = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => return Err(SaveLoadError::FileOpen),
    };
    if f.lines().count() < 1 {
        return Err(SaveLoadError::EmptyFile);
    }

    let y_max = f.lines().count();
    let x_max = f.lines().next().unwrap().chars().count();

    let characters: Vec<CellState> = f
        .chars()
        .filter_map(|c| match c {
            ALIVE_STATUS_CHARACTER => Some(CellState::Alive),
            DEAD_STATUS_CHARACTER => Some(CellState::Dead),
            _ => None,
        })
        .collect();

    let mut game = game::Game::new(x_max, y_max);
    game.replace_buffer(characters).unwrap();
    Ok(game)
}

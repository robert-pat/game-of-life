use crate::{game, text, ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER, GAME_X, GAME_Y};
use core::str;
use std::io::Read;

/// Reads a list of coordinates from a file.
pub fn file_to_coordinates(path: &str) -> Vec<(usize, usize)> {
    match std::fs::read_to_string(path) {
        Ok(contents) => text::parse_string_to_coordinates(contents),
        Err(_) => {
            eprintln!("Failed to read coordinates from file!");
            Vec::new()
        }
    }
}
/// Writes the given game board to the specified file.
/// This will replace the file if it already exists
pub fn save_board_to_file(path: &str, board: &game::GameBoard) {
    let mut contents: String = String::new();
    let space = board.space.clone();

    for row in space {
        for cell in row {
            contents.push(cell.into());
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
pub fn load_board_from_file(path: &str) -> game::GameBoard {
    let mut constructed_board: Vec<Vec<game::CellState>> = Vec::new();
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to load board from file");
            return game::GameBoard::new(GAME_X, GAME_Y);
        }
    };

    for row in contents.split('\n') {
        let mut constructed_row: Vec<game::CellState> = Vec::new();
        for s in row.chars() {
            constructed_row.push(match s {
                ALIVE_STATUS_CHARACTER => game::CellState::Alive,
                DEAD_STATUS_CHARACTER => game::CellState::Dead,
                _ => {
                    eprintln!("Error parsing char from file: [{}]", s);
                    continue;
                } //Don't push anything on error
            });
        }
        constructed_board.push(constructed_row);
    }

    game::GameBoard {
        x_max: constructed_board[0].len(),
        y_max: constructed_board.len(),
        space: constructed_board, // last to avoid borrowing after move
    }
}

/// Converts a raw text board from conwaylife.com into internal game representation
#[allow(unused)]
pub fn convert_wiki_to_board(path: &str) -> game::GameBoard {
    // Load the text from the file
    let mut file = std::fs::read_to_string(path).unwrap();

    let mut x_max: usize = 0;
    // Loop through each row of the file w/o an "!" in it; ! are comments in the Game of Life Wiki format
    for row in file.split('\n').filter(|r| !r.contains('!')) {
        let mut count = 0; // Counting how many cell characters there are in a row
        for c in row.chars() {
            match c {
                '.' | 'O' => count += 1,
                _ => {}
            }
        }
        if count > x_max {
            x_max = count; // Update the longest row
        }
    }

    let mut board = game::GameBoard::new(x_max, file.split('\n').count());
    let mut x = 0;
    let mut y = 0;

    // Clippy linting did this; idk if it works w/ these changes
    for (y, row) in file.split('\n').filter(|r| !r.contains('!')).enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '.' => board.set(x, y, game::CellState::Dead), // Technically unnecessary bc cells default dead
                'O' => board.set(x, y, game::CellState::Alive),
                _ => {}
            }
        }
    }
    board
}
/// Overwrites a conwaylife.com text board into a game save file
#[allow(unused)]
pub fn convert_wiki_file_to_save(path: &str) {
    let board = convert_wiki_to_board(path); // Load a board from the file
    save_board_to_file(path, &board); // Write the board to the original file
}

pub(crate) fn get_user_path() -> String {
    print!("Please enter a file:");
    let mut line = String::new();
    std::io::stdin()
        .read_to_string(&mut line)
        .expect("Failed to Read stdIn");
    line
}

pub(crate) fn load_from_path(path: &str) -> Option<game::GameBoard> {
    let data = match std::fs::metadata(path) {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };
    if !data.is_file() {
        return None;
    }
    Some(load_board_from_file(path))
}

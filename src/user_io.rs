use core::str;

use crate::{game, GAME_X, GAME_Y, ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};
use regex::Regex;
use lazy_static::lazy_static;

pub enum GameAction {
    Simulation,
    GrowCell,
    KillCell,
    PrintBoard,
    Quit,
    Play,
    Save,
    Failed
}

/// Prompts the user for a single coordinate
#[allow(unused)]
pub fn get_user_coordinate(std_in: &std::io::Stdin)->(usize, usize){
    let mut input:String = String::new();
    println!("Enter a coordinate of the form: x,y");
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    let nums: Vec<&str> = input.trim().split(",").collect();
    return (nums[0].parse().unwrap(), nums[1].parse().unwrap()); // TODO: properly handle parsing here
}

/// Prompts the user for any number of coordinates
pub fn get_user_coordinate_vec(std_in: &std::io::Stdin)-> Vec<(usize, usize)>{
    let mut input: String = String::new();

    println!("Please enter coordinates in the form: x,y x,y x,y ...");
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    return parse_string_to_coordinates(input);
}

/// Parses a given string into a Vec of coordinates, uses regex to match number,number patterns
/// This function handles parsing for get_user_coordinate_vec() & read_file_coordinates()
pub fn parse_string_to_coordinates(input: String)->Vec<(usize, usize)>{
    let mut cells: Vec<(usize, usize)> = Vec::new();

    // from the https://docs.rs/regex/latest/regex/ page
    lazy_static!{
        pub static ref FILTER: Regex = Regex::new(r"([\d]+,[\d]+)+").unwrap();
    }

    for c in FILTER.captures_iter(&input){
        let pair: Vec<_> = c.get(0).unwrap().as_str().split(",").collect();
        cells.push((pair[0].parse().unwrap(), pair[1].parse().unwrap()));
    }

    return cells;
}

/// Prompts the user for a single number
pub fn get_user_number(std_in: &std::io::Stdin)-> usize{
    let mut input:String = String::new();
    println!("Please enter a number:");
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    return match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Couldn't parse number");
            0
        }
    }
}

/// Reads a line from the console & parses it into a cust_io::Action
/// Does NOT prompt the user with a message
pub fn get_user_game_action(std_in: &std::io::Stdin) -> GameAction {
    println!("Pick an option:");
    println!("(s)imulate iterations, (g)row cells, (k)ill cells, (p)rint the board, (l)et the sim. run, sa(v)e the board, (q)uit/(c)ancel");
    let mut input: String = String::new();
    std_in.read_line(&mut input).expect("Failed reading stdIn");

    return match input.trim() {
        "s" => GameAction::Simulation,
        "p" => GameAction::PrintBoard,
        "g" => GameAction::GrowCell,
        "k" => GameAction::KillCell,
        "q" | "c" => GameAction::Quit,
        "l" => GameAction::Play,
        "v" => GameAction::Save,
        _ => GameAction::Failed
    }
}

/// Reads a list of coordinates from a file.
pub fn read_file_coordinates(path: &str) -> Vec<(usize, usize)>{
    return match std::fs::read_to_string(path) {
        Ok(contents) => parse_string_to_coordinates(contents),
        Err(_) => {
            eprintln!("Failed to read coordinates from file!");
            Vec::new()
        }
    }
}
/// Writes the given game board to the specified file.
/// This will replace the file if it already exists
pub fn save_board_to_file(path: &str, board: &game::GameBoard){
    let mut contents: String = String::new();
    let space = board.space.clone();

    for row in space{
        for cell in row{
            contents.push(cell.to_char());
        }
        contents.push('\n');
    }
    contents.pop(); // a newline in appended to the end of each row, even the last one (this removes it)
    match std::fs::write(path, contents){
        Ok(_) => println!("Saved Successfully!"),
        Err(_) => eprintln!("Error Saving Board")
    };
}

/// Loads a game board from a file.
/// If the file is improperly formatted, it will return an empty board.
/// Failing to load the board is logged to std err
pub fn load_board_from_file(path: String)-> game::GameBoard {
    let mut constructed_board: Vec<Vec<game::CellStatus>> = Vec::new();
    let contents = match std::fs::read_to_string(path){
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to load board from file");
            return game::GameBoard::new(GAME_X, GAME_Y);
        }
    };

    for row in contents.split("\n"){
        let mut constructed_row: Vec<game::CellStatus> = Vec::new();
        for s in row.chars(){
            constructed_row.push(match s{
                ALIVE_STATUS_CHARACTER => game::CellStatus::Alive,
                DEAD_STATUS_CHARACTER => game::CellStatus::Dead,
                _ => {eprintln!("Error parsing char from file: [{}]", s); continue;} //Don't push anything on error
            });
        }
        constructed_board.push(constructed_row);
    }

    return game::GameBoard {
        x_max: constructed_board[0].len(),
        y_max: constructed_board.len(),
        space: constructed_board, // last to avoid borrowing after move
    }
}

/// Converts a raw text board from conwaylife.com into internal game representation
#[allow(unused)]
pub fn convert_wiki_to_board(path: &str) -> game::GameBoard {
    // Load the text from the file
    let mut file = match std::fs::read_to_string(path){
        Ok(contents) => contents,
        Err(_) => {eprintln!("Error Converting board"); String::new()}
    };
  
    let mut x_max: usize = 0;
    // Loop through each row of the file w/o an "!" in it; ! are comments in the Game of Life Wiki format
    for row in file.split("\n").filter(|r| {!r.contains("!")}){
        let mut count = 0; // Counting how many cell characters there are in a row
        for c in row.chars(){
            match c {'.'| 'O' => count += 1, _ => {}}
        }
        if count > x_max{
            x_max = count; // Update the longest row
        } 
    }

    let mut board = game::GameBoard::new(x_max, file.split("\n").count());
    let mut x = 0; let mut y = 0;

    //Have to do the same thing bc cant borrow iterator in for loop (I think?)
    for row in file.split("\n").filter(|r| {!r.contains("!")}){
        x = 0;
        for c in row.chars(){
            match c{
                '.' => board.set(x, y, game::CellStatus::Dead), // Technically unnecessary bc cells default dead
                'O' => board.set(x, y, game::CellStatus::Alive),
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }
    return board;
}

/// Overwrites a conwaylife.com text board into a game save file
#[allow(unused)]
pub fn convert_wiki_file_to_save(path: &str){
    let board = convert_wiki_to_board(path); // Load a board from the file
    save_board_to_file(path, &board); // Write the board to the original file
}
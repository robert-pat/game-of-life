use std::io::Read;
use std::io::Write;

use ansi_escapes;

use crate::cust_io;
use crate::game;
use crate::debug;

pub fn initial_game_setup(x:usize, y:usize, std_in: &std::io::Stdin) -> game::Board{
    println!("Would you like to (s)tart normally or (l)oad from a file? You can press enter to start normally.");

    let mut new_board = game::Board::new(x, y);

    let mut input: String = String::new();
    std_in.read_line(&mut input);

    match input.trim(){
        "l" => {
            println!("File name?");
            input.clear();
            std_in.read_line(&mut input);

            new_board = cust_io::load_board_from_file(input.trim().to_string());

            return new_board;
        },
        _ => {
            println!("Please Enter a starting configuration:");
            let initial_cells = cust_io::get_user_coordinate_vec(&std_in);
            game::set_cells(&mut new_board, initial_cells, game::Status::Alive);
            return new_board;
        }
    }
}

pub fn debug_main(){
    println!("Running in Debug!");

    //println!("Line Rewriting Test: ------");
    //debug::line_rewriting_test();

    debug::file_convert_test();
}

// TODO: modify this function to remove the extra asking step: use coords if coords are typed & file otherwise
/// Gets a Vec<(usize, usize)> of cells to change from the user.
/// It will prompt them to type or read from a file.
pub fn promt_user_to_change_cells(std_in: &std::io::Stdin) -> Vec<(usize, usize)>{
    println!("Would you like to (t)ype in corodinates or (r)ead from a file?");

    let mut input: String = String::new();
    std_in.read_line(&mut input);

    match input.trim(){
        "t" => return cust_io::get_user_coordinate_vec(std_in),
        "r" => {
            println!("Enter the file name:");
            input.clear();
            std_in.read_line(&mut input);
            return cust_io::read_file_coordinates(input.trim().to_string());
        },
        _ => {
            eprintln!("Failed to parse.");
            return Vec::new();
        }
    }
}

/// Prints the board to the terminal, replacing previous text if replace_prev is true
pub fn display_next_iteratrion(board: &game::Board, std_out: &mut std::io::Stdout, replace_prev: bool, gen: usize){
    if(replace_prev){
        for _ in 0..=board.y_max{
            print!("{}", ansi_escapes::CursorPrevLine);
        }
    }
    print!("Generation: {gen}\n{board}");
    std_out.flush();
}

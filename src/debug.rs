#![allow(unused)]
use std::io::Write;

use crate::{game, GAME_X, GAME_Y};
use crate::user_io;
// TODO: convert the functions in this file to run as optional tests

#[derive(Debug)]
pub enum TestFailure{
    SavingLoading,
    WrongNeighborCount,
    WrongNeighbors
}

/// Test code to ensure file I/O works correctly
pub fn file_io_test() -> Result<(), TestFailure>{
    let mut board = game::GameBoard::new(10, 10);
    let cells = vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6),(7,7),(8,8),(9,9)];
    board.set_cells(cells, game::CellStatus::Alive);
    user_io::save_board_to_file("output.txt", &board);

    let loaded_board = user_io::load_board_from_file("output.txt".to_string());
    if board == loaded_board{
        Ok(())
    }
    else{
        Err(TestFailure::SavingLoading)
    }
}

// Test code to ensure overwriting of previous boards works correctly
pub fn line_rewriting_demo(){
    let mut std_out = std::io::stdout();
    print!("Here is a line.");
    std_out.flush().expect("Couldn't flush stdOut");
    print!("\r");
    print!("Here is the line rewritten!");
    std_out.flush().expect("Couldn't flush stdOut");
}

/// Test code to ensure converting wiki boards to save files works correctly
fn file_convert_test() -> Result<(), TestFailure>{
    //TODO: have an actual test here; need an example board that's already converted
    user_io::convert_wiki_to_board("test.txt");
    Ok(())
}

/// Test the neighboring cell code
pub fn print_all_raw_neighbors(){
    let board = game::GameBoard::new(5, 5);
    for y in 0..=board.y_max{
        for x in 0..=board.x_max{
            let neighbors = game::get_neighbors(&board, x, y);
            println!("({}, {}) --> {:?}", x, y, neighbors);
        }
    }
}

fn check_all_neighbor_counts() -> Result<(), TestFailure>{
    let board = game::GameBoard::new(GAME_X, GAME_Y);
    for y in 0..board.y_max{
        for x in 0..board.x_max{
            todo!() // Check count of neighbors: center has 8, corner has 3, edge has 5
        }
    }
    Ok(())
}

pub fn mini_find_neighbors_test() -> Result<(), TestFailure>{
    println!("Neighbors test 2");
    let board = game::GameBoard::new(5, 5);
    let cells = vec![
        (0,0),
        (0, board.y_max),
        (board.x_max, 0),
        (board.x_max, board.y_max),
        (2, 3),
        (0, 3),
        (3, 0),
        (1, 1),
    ];

    let answers = vec![3, 3, 3, 3, 8, 5, 5, 8];

    for i in 0..cells.len(){
        let cell = cells[i];
        if game::get_neighbors(&board, cell.0, cell.1).len() != answers[i]{
            return Err(TestFailure::WrongNeighborCount);
        }
    }
    Ok(())
}
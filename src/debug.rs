use std::io;
use std::io::Write;

use crate::game;
use crate::cust_io;
// TODO: convert the functions in this file to run as optional tests

/// Test code to ensure file I/O works correctly
pub fn file_io_test(){
    let mut board = game::Board::new(10, 10);
    let cells = vec!((0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6),(7,7),(8,8),(9,9));
    game::set_cells(&mut board, cells, game::Status::Alive);

    println!("Here is the existing board:\n{}", board);
    println!("It will be saved to: output.txt");

    cust_io::save_board_to_file("output.txt", board);
    let loaded_board = cust_io::load_board_from_file("output.txt".to_string());

    println!("Here is the loaded board:\n{}", loaded_board);
}

// Test code to ensure overwriting of previous boards works correctly
pub fn line_rewriting_test(){
    let mut std_out = std::io::stdout();
    print!("Here is a line.");
    std_out.flush();
    print!("\r");
    print!("Here is the line rewritten!");
    std_out.flush();
}

/// Test code to ensure converting wiki boards to save files works correctly
pub fn file_convert_test(){
    cust_io::convert_wiki_to_board("test.txt");
}

/// Test the neighboring cell code
pub fn find_neighbors_test(){
    println!("Pipe this to a text file, it'll be easier to verify");
    println!("Neighbors test 1");
    let board = game::Board::new(5, 5);
    for y in 0..=board.y_max{
        for x in 0..=board.x_max{
            let neighbors = game::get_neighbors(&board, x, y);
            println!("({}, {}) --> {:?}", x, y, neighbors);
        }
    }
}

pub fn rand_find_neighbors_test(){
    println!("Neighbors test 2");
    let board = game::Board::new(5, 5);
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

    for c in cells{
        println!("({}, {}) --> {:?}", c.0, c.1, game::get_neighbors(&board, c.0, c.1));
    }
}
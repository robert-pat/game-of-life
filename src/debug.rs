use std::io;
use std::io::Write;

use crate::game;
use crate::cust_io;

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

pub fn line_rewriting_test(){
    let mut std_out = std::io::stdout();
    print!("Here is a line.");
    std_out.flush();
    print!("\r");
    print!("Here is the line rewritten!");
    std_out.flush();
}

pub fn file_convert_test(){
    cust_io::convert_wiki_to_board("test.txt");
}
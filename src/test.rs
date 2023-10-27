#[cfg(test)]
use std::io::Write;
#[cfg(test)]
use crate::game;

#[cfg(test)]
use crate::{GAME_X, GAME_Y};
#[cfg(test)]
use crate::save_load;

#[test]
pub fn file_io_test() {
    let mut board = game::GameBoard::new(10, 10);
    let cells = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8), (9, 9)];
    board.set_cells(cells, game::CellState::Alive);
    save_load::save_board_to_file("output.txt", &board);

    let loaded_board = save_load::load_board_from_file("output.txt");
    assert!(board == loaded_board)
}

#[test]
fn file_convert_test(){
    save_load::convert_wiki_to_board("test.txt");
    todo!() // Need to have an actual test here; need an example board that's already converted
}

#[test]
fn check_all_neighbor_counts(){
    let board = game::GameBoard::new(GAME_X, GAME_Y);
    for y in 0..board.y_max{
        for x in 0..board.x_max{
            let count = game::get_neighbors(&board, x, y).len();

            if (x == 0 || x == board.x_max) && (y == 0 || y == board.y_max) {
                assert_eq!(count , 3, "({x},{y}) failed"); // Corner cell
            }
            else if  x == 0 || y == 0 || x ==board.x_max || y == board.y_max{
                assert_eq!(count, 5, "({x},{y}) failed"); // Edge cell
            }
            else{
                assert_eq!(count, 8, "({x},{y}) failed"); // Center / "normal" cell
            }
        }
    }
}

#[test]
pub fn mini_find_neighbors_test(){
    let board = game::GameBoard::new(5, 5);
    let cells = [(0,0),
        (0, board.y_max),
        (board.x_max, 0),
        (board.x_max, board.y_max),
        (2, 3),
        (0, 3),
        (3, 0),
        (1, 1)];

    let answers = [3, 3, 3, 3, 8, 5, 5, 8];

    for i in 0..cells.len(){
        let cell = cells[i];
        assert_eq!(
            game::get_neighbors(&board, cell.0, cell.1).len(),
            answers[i],
            "{:?} failed)",
            cell
        )
    }
}

#[test]
pub fn dead_board_test(){
    let mut board = game::GameBoard::new(10, 10);
    assert!(!board.has_alive_cells());

    board.set(5, 5, game::CellState::Alive);
    assert!(board.has_alive_cells());
}
#[test]
// Test code to ensure overwriting of previous boards works correctly
pub fn line_rewriting_demo(){
    let mut std_out = std::io::stdout();
    print!("Here is a line.");
    std_out.flush().expect("Couldn't flush stdOut");
    print!("\r");
    print!("Here is the line rewritten!");
    std_out.flush().expect("Couldn't flush stdOut");
}
#[test]
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
#[test]
pub fn display_board_rewriting(){
    let mut board = game::GameBoard::new(10, 10);

    println!("{}", board);
    for _ in 0..board.y_max{
        print!("{}", ansi_escapes::CursorPrevLine);
    }

    std::thread::sleep(std::time::Duration::from_millis(1000));

    let mut vec = Vec::new();
    for y in 0..board.y_max{
        for x in 0..board.x_max{
            vec.push((x,y));
        }
    }
    board.set_cells(vec, game::CellState::Alive);
    print!("{}", board);
    if std::io::stdout().flush().is_ok() {}
}
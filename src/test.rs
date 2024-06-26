#[cfg(test)]
use crate::game;
#[cfg(test)]
use std::io::Write;

#[cfg(test)]
use crate::game::CellState;
#[cfg(test)]
use crate::save_load;
#[cfg(test)]
use crate::{GAME_X, GAME_Y};

#[test]
pub fn file_io_test() {
    let mut board = game::GameBoardOld::new(10, 10);
    let cells = vec![
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    ];
    board.set_cells(cells, CellState::Alive);
    save_load::save_board("test-tmp.txt", &board);

    let loaded_board = save_load::load_board_from_file("test-tmp.txt");
    assert!(board == loaded_board)
}
#[test]
fn check_all_neighbor_counts() {
    let board = game::GameBoardOld::new(GAME_X, GAME_Y);
    for y in 0..board.y_max {
        for x in 0..board.x_max {
            let count = game::get_neighbors(&board, x, y).len();

            if (x == 0 || x == board.x_max) && (y == 0 || y == board.y_max) {
                assert_eq!(count, 3, "({x},{y}) failed"); // Corner cell
            } else if x == 0 || y == 0 || x == board.x_max || y == board.y_max {
                assert_eq!(count, 5, "({x},{y}) failed"); // Edge cell
            } else {
                assert_eq!(count, 8, "({x},{y}) failed"); // Center / "normal" cell
            }
        }
    }
}

#[test]
pub fn mini_find_neighbors_test() {
    let board = game::GameBoardOld::new(5, 5);
    let cells = [
        (0, 0),
        (0, board.y_max),
        (board.x_max, 0),
        (board.x_max, board.y_max),
        (2, 3),
        (0, 3),
        (3, 0),
        (1, 1),
    ];

    let answers = [3, 3, 3, 3, 8, 5, 5, 8];

    for i in 0..cells.len() {
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
pub fn dead_board_test() {
    let mut board = game::GameBoardOld::new(10, 10);
    assert!(!board.has_alive_cells());

    board.set(5, 5, CellState::Alive);
    assert!(board.has_alive_cells());
}
#[test]
/// Test the neighboring cell code
pub fn print_all_raw_neighbors() {
    let board = game::GameBoardOld::new(5, 5);
    let mut s = String::with_capacity(25 * 40); // ~40 chars per cell ?
    for y in 0..=board.y_max {
        for x in 0..=board.x_max {
            let neighbors = game::get_neighbors(&board, x, y);
            // Love it when what ever this is?
            s += &*format!("({}, {}) --> {:?}", x, y, neighbors);
        }
    }
    std::fs::write("test-temp.txt", s).unwrap();
}
#[test]
pub fn display_board_rewriting() {
    let mut board = game::GameBoardOld::new(10, 10);

    println!("{}", board);
    for _ in 0..board.y_max {
        print!("{}", ansi_escapes::CursorPrevLine);
    }

    std::thread::sleep(std::time::Duration::from_millis(1000));

    let mut vec = Vec::new();
    for y in 0..board.y_max {
        for x in 0..board.x_max {
            vec.push((x, y));
        }
    }
    board.set_cells(vec, CellState::Alive);
    print!("{}", board);
    if std::io::stdout().flush().is_ok() {}
}

#[test]
fn new_board_updating() {
    let mut board = game::Game::new(10, 10);
    let cells = [(1, 1), (2, 2), (3, 3), (4, 4)];
    board.set_many(&cells, &[CellState::Alive]);

    for _ in 0..3 {
        eprintln!("{board}");
        board.step(1);
    }
}

use crate::{ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};

/// Represents a Game of Life board and its dimensions
#[derive(Clone)]
pub struct GameBoard {
    pub space: Vec<Vec<Status>>,
    pub x_max: usize,
    pub y_max: usize
}

/// Holds the possible states each cell can have
#[derive(Clone, Copy)]
pub enum Status{
    Alive,
    Dead
}
impl Status{
    /// Converts a Status into its character representation
    pub fn to_char(&self)-> char{
        match self{
            Status::Alive => ALIVE_STATUS_CHARACTER,
            Status::Dead => DEAD_STATUS_CHARACTER
        }
    }
}
impl std::fmt::Debug for Status{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f, "{}",  match self{Status::Dead => DEAD_STATUS_CHARACTER, Status::Alive => ALIVE_STATUS_CHARACTER})
    }
}

impl GameBoard {
    /// Gets the Status of a specific cell on the board
    pub fn get(&self, x: usize, y: usize)->Status{
        let x_ = x % self.x_max;
        let y_ = y % self.y_max;
        return self.space[y_][x_];
    }

    /// Sets the Status of a specific cell on the board
    pub fn set(&mut self, x: usize, y: usize, value: Status){
        let x_ = x % self.x_max;
        let y_ = y % self.y_max;
        self.space[y_][x_] = value;
    }

    /// Creates a new board with the specified dimensions.
    /// This function also fills in the board to be the specific size
    pub fn new(x: usize, y: usize)-> Self{
        Self{
            space: vec![vec![Status::Dead; x]; y],
            x_max: x,
            y_max: y
        }
    }

    ///Returns whether the board has any Alive cells in it
    pub fn has_alive_cells(&self) -> bool{
        for row in &self.space{
            for cell in row{
                match cell{
                    Status::Alive => return true,
                    _ => {}
                }
            }
        }
        return false;
    }
}
impl std::fmt::Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        let mut s: String = String::new();

        for entry in self.space.iter(){
            s.push_str( format!("{:?}", entry).as_str() );
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}

/// Returns a vector of tuples containing the coordinates of all the given cell's neighbors
pub fn get_neighbors(board: &GameBoard, x: usize, y: usize) -> Vec<(usize, usize)>{
    let mut result: Vec<(usize, usize)> = Vec::new();
    // Have to check for the zero conditions b/c usize can't be negative; panics if 0 - 1
    if x == 0 && y == 0{ //Origin cell
        result.push((1, 0));
        result.push((1, 1));
        result.push((0, 1));
    }
    else if x == 0{
        for _y in (y -1)..=(y +1){
            for _x in x..=(x +1){
                if _y == y && _x == x{continue;}
                if _y > board.y_max || _x >board.x_max{ continue; }
                result.push((_x, _y));
            }
        }
    }
    else if y == 0{
        for _y in y..=(y +1){
            for _x in (x -1)..=(x +1){
                if _y == y && _x == x{continue;}
                if _y > board.y_max || _x >board.x_max{ continue; }
                result.push((_x, _y));
            }
        }
    }
    else{ // Non zero-bordering cell
        for _y in (y -1)..=(y+ 1){
            for _x in (x-1)..=(y +1){
                if _y == y && _x == x{continue;}
                if _y > board.y_max || _x >board.x_max{ continue; }
                result.push((_x, _y));
            }
        }
    }
    return result;
}

/// Returns the next iteration of the given board, w/ the same dimensions
fn update_board(old_board: &GameBoard) -> GameBoard {
    let mut new_board = GameBoard::new(old_board.x_max, old_board.y_max);

    for y in 0..=old_board.y_max{
        for x in 0..=old_board.x_max{
            // Only 2 cases where cells need to be alive on the new board
            match old_board.get(x, y){
                // When the cell is alive, it dies w/o having 2 or 3 neighbors
                Status::Alive => {
                    match num_alive_neighbors(&old_board, x, y){
                        2 | 3 => new_board.set(x, y, Status::Alive),
                        _ => {} // Cells are dead by default in the new board
                    }
                },
                // When the cell is dead, it needs 3 alive neighbors to revive
                Status::Dead => {
                    match num_alive_neighbors(&old_board, x, y){
                        3 => new_board.set(x, y, Status::Alive),
                        _ => {} // Cells are dead by default in the new board
                    }
                }
            }
        }
    }
    return new_board;
}

/// Counts the number of living neighbors a given cell has; as a usize
pub fn num_alive_neighbors(board: &GameBoard, x: usize, y: usize) -> usize{
    let mut count: usize = 0;
    for cell in get_neighbors(&board, x, y){ // Loop through neighbor cells
        match board.get(cell.0, cell.1){
            Status::Alive => count += 1,
            Status::Dead => {}
        }
    }
    return count;
}

pub fn set_cells(board: &mut GameBoard, cells_to_change: Vec<(usize, usize)>, status: Status){
    for s in cells_to_change{
        board.set(s.0, s.1, status);
    }
}

pub fn run_iterations(board: &GameBoard, n: usize) -> GameBoard {
    let mut new_board: GameBoard = board.clone();
    for _ in 0..n{
        new_board = update_board(&new_board);
    }
    return new_board;
}
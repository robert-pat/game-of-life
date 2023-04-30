use crate::{ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};

/// Represents a Game of Life board and its dimentions
#[derive(Clone)]
pub struct Board{
    pub space: Vec<Vec<Status>>,
    pub x_max: usize,
    pub y_max: usize
}

/// Holds the posible states each cell can have
#[derive(Clone, Copy)]
pub enum Status{
    Alive,
    Dead
}
impl Status{
    pub fn to_char(&self)-> char{
        match self{
            Status::Alive => ALIVE_STATUS_CHARACTER,
            Status::Dead => DEAD_STATUS_CHARACTER
        }
    }
}
impl std::fmt::Debug for Status{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f, "{}", if matches!(self, Status::Dead) {DEAD_STATUS_CHARACTER} else{ALIVE_STATUS_CHARACTER})
    }
}

impl Board{
    pub fn get(&self, x: usize, y: usize)->Status{
        let mut x_ = x % self.x_max;
        let mut y_ = y % self.y_max;
        return self.space[y_][x_];
    }

    pub fn set(&mut self, x: usize, y: usize, value: Status){
        let mut x_ = x % self.x_max;
        let mut y_ = y % self.y_max;
        self.space[y_][x_] = value;
    }

    pub fn new(x: usize, y: usize)-> Self{
        let mut collection = vec![vec![Status::Dead; x]; y];

        let mut game_board = Board{
            space: collection,
            x_max: x,
            y_max: y
        };
          
        return game_board;
    }

///Returns whether the board has any Alive cells in it
    pub fn has_alive_cells(&self) -> bool{
        for row in &self.space{
            for cell in row{
                if matches!(Status::Alive, cell){
                    return true;
                }
            }
        }
        return false;
    }
}
impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        let mut s: String = String::new();

        for entry in self.space.iter(){
            s.push_str( format!("{:?}", entry).as_str() );
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}



/// Returs a vector of tuples containing the coordinates of all the given cell's neighbors
pub fn get_neighbors(board: &Board, x: usize, y: usize) -> Vec<(usize, usize)>{
    let mut result: Vec<(usize, usize)> = Vec::new();

    if x > 0 && y > 0{ // most common case; 8 neighbors w/o any bounds 
        for _y in y-1..=y+1{
            for _x in x-1..=x+1{
                if x > board.x_max || y > board.y_max{ continue; }
                if x == _x && y == _y{ continue; }
                result.push((_x, _y)); 
            }
        }
    }
    else if x == 0 && y == 0{ // smallest case at (0,0)
        result.push((1, 0));
        result.push((0, 1));
        result.push((1, 1));
    }
    // these are the border cases; only 1 coordinate is 0
    else if x == 0{
        for _y in y-1..=y+1{
            for _x in x..=x+1{
                if _x > board.x_max || _y > board.y_max{ continue; }
                if _x == x && _y == y{ continue; }
                result.push((_x, _y));
            }
        }
    }
    else if y == 0{
        for _y in y..=y+1{
            for _x in x-1..=x+1{
                if _x > board.x_max || _y > board.y_max{ continue; }
                if _x == x && _y == y{ continue; }
                result.push((_x, _y));
            }
        }
    }

    return result;
}

/// Returns the next iteration of the given board, w/ the same dimentions
fn update_board(old_board: &Board)->Board{
    let mut new_board = Board::new(old_board.x_max, old_board.y_max);

    for y in 0..=old_board.y_max{
        for x in 0..=old_board.x_max{
            // Handle the cell being dead: revive the cell (if possible) or leave it dead
            if matches!(old_board.get(x, y), Status::Dead){
                if num_alive_neighbors(&old_board, x, y) == 3 {
                    new_board.set(x, y, Status::Alive)
                } 
                else {
                    new_board.set(x, y, Status::Dead)
                }
                continue;
            }
            // set the next state from the current number of neighbors
            match num_alive_neighbors(&new_board, x, y){
                0 | 1 => new_board.set(x, y, Status::Dead),
                2 | 3 => new_board.set(x, y, Status::Alive),
                4| 5| 6| 7| 8| 9 => new_board.set(x, y, Status::Dead),
                _ => println!("Error matching cell neighbor count")
            }
        }
    }
    return new_board;
}

/// Counts the number of living neighbors a given cell has; as a usize
pub fn num_alive_neighbors(board: &Board, x: usize, y: usize) -> usize{
    let mut count: usize = 0;

    // loop through the cells surrounding the target (x, y)
    for cell in get_neighbors(&board, x, y){
        if matches!(board.get(cell.0, cell.1), Status::Alive){
            count += 1;
        }
    }

    return count;
}

pub fn set_cells(board: &mut Board, cells_to_change: Vec<(usize, usize)>, status: Status){
    for s in cells_to_change{
        board.set(s.0, s.1, status);
    }
}

pub fn run_iterations(board: &Board, n: usize) -> Board{
    let mut new_board: Board = board.clone();
    for _ in 0..n{
        new_board = update_board(&new_board);
    }
    return new_board;
}
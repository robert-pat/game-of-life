use crate::{ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead
}
#[allow(clippy::from_over_into)] // dumb lint
impl Into<char> for CellState{
    fn into(self) -> char {
        match self {
            CellState::Alive => ALIVE_STATUS_CHARACTER,
            CellState::Dead => DEAD_STATUS_CHARACTER
        }
    }
}
impl std::fmt::Debug for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f, "{}",  match self{
            CellState::Dead => DEAD_STATUS_CHARACTER,
            CellState::Alive => ALIVE_STATUS_CHARACTER
        })
    }
}
#[derive(Clone)]
pub struct GameBoard {
    pub space: Vec<Vec<CellState>>,
    pub x_max: usize,
    pub y_max: usize
}
impl GameBoard {
    /// Gets the Status of a specific cell on the board
    pub fn get(&self, x: usize, y: usize)-> CellState {
        let _x = x % self.x_max;
        let _y = y % self.y_max;
        self.space[_y][_x]
    }
    /// Sets the Status of a specific cell on the board
    pub fn set(&mut self, x: usize, y: usize, value: CellState){
        let x_ = x % self.x_max;
        let y_ = y % self.y_max;
        self.space[y_][x_] = value;
    }
    /// Sets the Status of the cells on the board
    pub fn set_cells(&mut self, cells: Vec<(usize, usize)>, status: CellState){
        for cell in cells {
            let x = cell.0 % self.x_max;
            let y = cell.1 % self.y_max;
            self.space[y][x] = status;
        }
    }
    /// Creates a new board with the specified dimensions.
    /// This function also fills in the board to be the specific size
    pub fn new(x: usize, y: usize)-> Self{
        Self{
            space: vec![vec![CellState::Dead; x]; y],
            x_max: x,
            y_max: y
        }
    }
    ///Returns whether the board has any Alive cells in it
    pub fn has_alive_cells(&self) -> bool{
        for row in &self.space{
            if row.contains(&CellState::Alive) {return true;}
        }
        false
    }
    pub fn clear(&mut self){
        for r in self.space.iter_mut(){
            for s in r { *s = CellState::Dead; }
        }
    }
    pub(crate) fn update_to(&self, other: &mut GameBoard){
        other.clear();
        for (y, row) in self.space.iter().enumerate(){
            for (x, cell) in row.iter().enumerate(){
                let neighbors = num_alive_neighbors(self, x, y);
                use CellState as S;
                let new_cell = match cell{
                    S::Alive => { if neighbors == 2 || neighbors == 3 {S::Alive} else {S::Dead}},
                    S::Dead => { if neighbors == 3 {S::Alive} else {S::Dead} },
                };
                other.set(x, y, new_cell);
            }
        }
    }
}
impl std::fmt::Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result{
        let mut s: String = String::new();

        for row in self.space.iter(){
            s.push_str( format!("{:?}", row).as_str() );
            s.push('\n');
        }
        s.pop(); //Remove the last trailing new line
        write!(f, "{}", s)
    }
}
impl PartialEq for GameBoard{
    fn eq(&self, other: &Self) -> bool {
        if self.y_max != other.y_max || self.x_max != other.x_max { return false; }
        for y in 0..self.y_max{
            for x in 0..self.x_max{
                if self.space[y][x] != other.space[y][x]{return false;}
            }
        }
        true
    }
}
/// Returns a vector of tuples containing the coordinates of all the given cell's neighbors
/// If the given coordinates are outside of the board, it will return an empty vec
pub fn get_neighbors(board: &GameBoard, x: usize, y: usize) -> Vec<(usize, usize)>{
    /*
    let (x, y) = (x as i32, y as i32);
    let (x_m, y_m) = (board.x_max as i32, board.y_max as i32);
    let mut points = vec![
        (x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)
    ];
    points.retain(|point|
        {point.0 >= 0 && point.0 <= x_m && point.1 >= 0 && point.1 <= y_m}
    );
    let mut neighbors = Vec::with_capacity(points.len());
    for p in points{ neighbors.push((p.0 as usize, p.1 as usize)); }
*/
    // Number of refactors this function has had: ||||||||
    // I swear to god this has made me lose interest in this projects at least 3 times
    // I hate it so much
    // update: 10/26/2023 -> skill issue, its like much neater!
    if x > board.x_max || y > board.y_max { vec![] }

    else if x == 0 && y == 0{ vec![(0, 1), (1, 1), (1, 0)] }
    else if x == 0 && y == board.y_max{ vec![(x, y -1), (x +1, y -1), (x +1, y)] }
    else if x == board.x_max && y == 0{ vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)] }
    else if x == board.x_max && y == board.y_max{ vec![(x - 1, y - 1), (x - 1, y), (x, y - 1)] }

    else if x == 0{
        vec![(x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
    }
    else if x == board.x_max{
        vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1)]
    }
    else if y == 0{
        vec![(x - 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y), (x + 1, y + 1)]
    }
    else if y == board.y_max{
        vec![(x - 1, y - 1), (x - 1, y), (x, y - 1), (x + 1, y - 1), (x + 1, y)]
    }

    else{
        vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
    }
}

/// Returns the next iteration of the given board, w/ the same dimensions
fn update_board(old_board: &GameBoard) -> GameBoard {
    let mut new_board = GameBoard::new(old_board.x_max, old_board.y_max);

    for y in 0..=old_board.y_max{
        for x in 0..=old_board.x_max{
            // Only 2 cases where cells need to be alive on the new board
            match old_board.get(x, y){
                // When the cell is alive, it dies w/o having 2 or 3 neighbors
                CellState::Alive => {
                    match num_alive_neighbors(old_board, x, y){
                        2 | 3 => new_board.set(x, y, CellState::Alive),
                        _ => {} // Cells are dead by default in the new board
                    }
                },
                // When the cell is dead, it needs 3 alive neighbors to revive
                CellState::Dead => {
                    if num_alive_neighbors(old_board, x, y) == 3 {
                        new_board.set(x, y, CellState::Alive)
                    }
                }
            }
        }
    }
    new_board
}

/// Counts the number of living neighbors a given cell has; as a usize
pub fn num_alive_neighbors(board: &GameBoard, x: usize, y: usize) -> usize{
    let mut count: usize = 0;
    for cell in get_neighbors(board, x, y){ // Loop through neighbor cells
        match board.get(cell.0, cell.1){
            CellState::Alive => count += 1,
            CellState::Dead => {}
        }
    }
    count
}

/// Returns the board after n iterations
pub fn run_iterations(board: &GameBoard, n: usize) -> GameBoard {
    let mut new_board: GameBoard = board.clone();
    for _ in 0..n{
        new_board = update_board(&new_board);
    }
    new_board
}
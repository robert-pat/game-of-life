use crate::{ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead
}
#[allow(clippy::from_over_into)] // dumb lint
impl From<CellState> for char{
    fn from(value: CellState) -> Self {
        match value{
            CellState::Alive => ALIVE_STATUS_CHARACTER,
            CellState::Dead => DEAD_STATUS_CHARACTER,
        }
    }
}
impl std::fmt::Debug for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        // Normal programming language with sensical syntax & errors:
        // "let c: char = *self.into();" failed, as did just *self.into(),
        write!(f, "{}", <CellState as Into<char>>::into(*self))
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
        self.space[y % self.y_max][x % self.x_max]
    }
    /// Sets the Status of a specific cell on the board
    pub fn set(&mut self, x: usize, y: usize, value: CellState){
        self.space[y % self.y_max][x % self.x_max] = value;
    }
    /// Sets the Status of the cells on the board
    pub fn set_cells(&mut self, cells: Vec<(usize, usize)>, status: CellState){
        for cell in cells {
            self.space[cell.1 % self.y_max][cell.0 % self.x_max] = status;
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
        for (y, row) in self.space.iter().enumerate(){
            if &other.space[y] != row {return false;}
        }
        true
    }
}
/// Returns a vector with the coordinates of all the given cell's neighbors
/// If the given coordinates are outside of the board, it will return an empty vec
pub fn get_neighbors(board: &GameBoard, x: usize, y: usize) -> Vec<(usize, usize)>{
    /* Number of refactors this function has had: ||||||||
    * swear to god this has made me lose interest in this projects at least 3 times, I hate it so much
    * update: 10/26/2023 -> skill issue, its like much neater! */
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
    neighbors
}
/// Returns the next iteration of the given board, w/ the same dimensions
fn update_board(old_board: &GameBoard) -> GameBoard {
    let mut new_board = GameBoard::new(old_board.x_max, old_board.y_max);
    for (y, row) in old_board.space.iter().enumerate(){
        for (x, cell) in row.iter().enumerate(){
            let neighbors = num_alive_neighbors(old_board, x, y);
            match cell{
                CellState::Alive if neighbors == 2 || neighbors == 3 => new_board.set(x, y, CellState::Alive),
                CellState::Dead if neighbors == 3 => new_board.set(x, y, CellState::Alive),
                _ => {}, // Cells are dead by default
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
use crate::{ALIVE_STATUS_CHARACTER, DEAD_STATUS_CHARACTER};
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[allow(unused)]
pub enum GameAction {
    Step,
    GrowCell,
    KillCell,
    PrintBoard,
    Quit,
    Play,
    Save,
    Failed,
    Paused,
}
impl std::fmt::Display for GameAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}
#[allow(clippy::from_over_into)] // dumb lint
impl From<CellState> for char {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Alive => ALIVE_STATUS_CHARACTER,
            CellState::Dead => DEAD_STATUS_CHARACTER,
        }
    }
}
impl std::fmt::Debug for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Normal programming language with sensical syntax & errors:
        // "let c: char = *self.into();" failed, as did just *self.into(),
        write!(f, "{}", <CellState as Into<char>>::into(*self))
    }
}

/// This is the new type representing a board playing the Game of Life.
/// The old code is left untill more refactoring can happen, but using the new
/// one is recommoned.
///
/// The GUI is updated to use the new one.
#[derive(Debug, Clone)]
pub struct Game {
    x_max: usize,
    y_max: usize,
    current: Vec<CellState>,
    previous: Vec<CellState>,
}
impl Game {
    pub fn new(x: usize, y: usize) -> Self {
        Game {
            x_max: x,
            y_max: y,
            current: vec![CellState::Dead; x * y],
            previous: vec![CellState::Dead; x * y],
        }
    }
    #[allow(unused)]
    pub fn get(&self, x: usize, y: usize) -> Option<CellState> {
        if !(0..self.x_max).contains(&x) || !(0..self.y_max).contains(&y) {
            return None;
        }
        Some(self.current[y * self.y_max + x])
    }
    #[allow(unused)]
    pub fn set(&mut self, x: usize, y: usize, cell: CellState) {
        assert!((0..self.x_max).contains(&x) && (0..self.y_max).contains(&y));
        self.current[y * self.y_max + x] = cell;
    }
    #[allow(unused)]
    pub fn set_many(&mut self, pos: &[(usize, usize)], cells: &[CellState]) {
        assert_eq!(pos.len(), cells.len());
        for (p, c) in pos.iter().zip(cells) {
            let (x, y) = p;
            self.current[y * self.y_max + x] = *c;
        }
    }
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.current.iter_mut().for_each(|c| *c = CellState::Dead);
        self.previous.iter_mut().for_each(|c| *c = CellState::Dead);
    }
    fn iterate(&mut self) {
        let (x_max, y_max) = (self.x_max as i32, self.y_max as i32);
        for (cell_index, cell) in self.current.iter().enumerate() {
            let (x, y) = (
                (cell_index % self.y_max) as i32,
                (cell_index / self.y_max) as i32,
            );
            let mut neighbors = vec![
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ];
            neighbors.retain(|(x, y)| (0..x_max).contains(x) && (0..y_max).contains(y));
            let alive_neighbors: usize = neighbors
                .into_iter()
                .map(|(x, y)| self[(x as usize, y as usize)])
                .map(|cell| if cell == CellState::Alive { 1 } else { 0 })
                .sum();

            self.previous[cell_index] = match cell {
                CellState::Dead if alive_neighbors == 3 => CellState::Alive,
                CellState::Alive if alive_neighbors == 2 => CellState::Alive,
                CellState::Alive if alive_neighbors == 3 => CellState::Alive,
                _ => CellState::Dead,
            };
        }
        std::mem::swap(&mut self.current, &mut self.previous);
    }
    pub fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            self.iterate();
        }
    }
    pub fn clone_from_old(&mut self, old: &GameBoardOld) -> Result<(), ()> {
        if self.x_max != old.x_max || self.y_max != old.y_max {
            return Err(());
        }
        for (index, cell) in self.current.iter_mut().enumerate() {
            let (x, y) = (index % self.y_max, index / self.y_max);
            *cell = old.get(x, y);
        }
        Ok(())
    }
}
impl std::ops::Index<(usize, usize)> for Game {
    type Output = CellState;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;

        assert!((0..self.x_max).contains(&x) && (0..self.y_max).contains(&y));
        &self.current[self.y_max * y + x]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Game {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;

        assert!((0..self.x_max).contains(&x) && (0..self.y_max).contains(&y));
        &mut self.current[self.y_max * y + x]
    }
}
impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.x_max != other.x_max {
            return false;
        }
        if self.y_max != other.y_max {
            return false;
        }
        self.current == other.current
    }
}

#[derive(Clone)]
pub struct GameBoardOld {
    pub space: Vec<Vec<CellState>>,
    pub x_max: usize,
    pub y_max: usize,
}
#[allow(unused)]
impl GameBoardOld {
    /// Gets the Status of a specific cell on the board
    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.space[y % self.y_max][x % self.x_max]
    }
    /// Sets the Status of a specific cell on the board
    pub fn set(&mut self, x: usize, y: usize, value: CellState) {
        self.space[y % self.y_max][x % self.x_max] = value;
    }
    /// Sets the Status of the cells on the board
    pub fn set_cells(&mut self, cells: Vec<(usize, usize)>, status: CellState) {
        for cell in cells {
            self.space[cell.1 % self.y_max][cell.0 % self.x_max] = status;
        }
    }
    /// Creates a new board with the specified dimensions.
    /// This function also fills in the board to be the specific size
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            space: vec![vec![CellState::Dead; x]; y],
            x_max: x,
            y_max: y,
        }
    }
    ///Returns whether the board has any Alive cells in it
    pub fn has_alive_cells(&self) -> bool {
        for row in &self.space {
            if row.contains(&CellState::Alive) {
                return true;
            }
        }
        false
    }
    pub fn clear(&mut self) {
        for r in self.space.iter_mut() {
            for s in r {
                *s = CellState::Dead;
            }
        }
    }
    pub(crate) fn update_to(&self, other: &mut GameBoardOld) {
        other.clear();
        for (y, row) in self.space.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let neighbors = num_alive_neighbors(self, x, y);
                use CellState as S;
                let new_cell = match cell {
                    S::Alive => {
                        if neighbors == 2 || neighbors == 3 {
                            S::Alive
                        } else {
                            S::Dead
                        }
                    }

                    S::Dead => {
                        if neighbors == 3 {
                            S::Alive
                        } else {
                            S::Dead
                        }
                    }
                };
                other.set(x, y, new_cell);
            }
        }
    }
    pub(crate) fn rescale_bounds(&mut self) {
        self.y_max = self.space.len();
        self.x_max = self.space[0].len();
    }
}
impl std::fmt::Display for GameBoardOld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.space.iter() {
            writeln!(f, "{:?}", row)?
        }
        Ok(())
    }
}
impl PartialEq for GameBoardOld {
    fn eq(&self, other: &Self) -> bool {
        if self.y_max != other.y_max || self.x_max != other.x_max {
            return false;
        }
        for (y, row) in self.space.iter().enumerate() {
            if &other.space[y] != row {
                return false;
            }
        }
        true
    }
}
/// Returns a vector with the coordinates of all the given cell's neighbors
/// If the given coordinates are outside of the board, it will return an empty vec
pub fn get_neighbors(board: &GameBoardOld, x: usize, y: usize) -> Vec<(usize, usize)> {
    /* Number of refactors this function has had: ||||||||
     * swear to god this has made me lose interest in this projects at least 3 times, I hate it so much
     * update: 10/26/2023 -> skill issue, its like much neater! */
    let (x, y) = (x as i32, y as i32);
    let (x_m, y_m) = (board.x_max as i32, board.y_max as i32);
    let mut points = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    points.retain(|p| (0..=x_m).contains(&p.0) && (0..=y_m).contains(&p.1));
    points
        .into_iter()
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}
/// Returns the next iteration of the given board, w/ the same dimensions
fn update_board(old_board: &GameBoardOld) -> GameBoardOld {
    let mut new_board = GameBoardOld::new(old_board.x_max, old_board.y_max);
    for (y, row) in old_board.space.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbors = num_alive_neighbors(old_board, x, y);
            match cell {
                CellState::Alive if neighbors == 2 || neighbors == 3 => {
                    new_board.set(x, y, CellState::Alive)
                }
                CellState::Dead if neighbors == 3 => new_board.set(x, y, CellState::Alive),
                _ => {} // Cells are dead by default
            }
        }
    }
    new_board
}

/// Counts the number of living neighbors a given cell has; as a usize
pub fn num_alive_neighbors(board: &GameBoardOld, x: usize, y: usize) -> usize {
    let mut count: usize = 0;
    for cell in get_neighbors(board, x, y) {
        // Loop through neighbor cells
        match board.get(cell.0, cell.1) {
            CellState::Alive => count += 1,
            CellState::Dead => {}
        }
    }
    count
}
/// Returns the board after n iterations
pub fn run_iterations(board: &GameBoardOld, n: usize) -> GameBoardOld {
    let mut new_board: GameBoardOld = board.clone();
    for _ in 0..n {
        new_board = update_board(&new_board);
    }
    new_board
}

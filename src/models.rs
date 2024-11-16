/// X = 1 = even
/// O = -1 = odd
/// Describes the state of one cell, it can be X, O, None or Expiring
/// Expiring is seperate because it is not counted towards a win
/// but also inhibits making a turn at that cell
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    X, O, E, N
}
impl Cell {
    /// numeric representation to be passed to javascript
    /// only to be used in #[wasm-bindgen] functions, 
    /// otherwise useless
    pub fn number(&self) -> i32 {
        match self {
            Cell::X => 1,
            Cell::O => -1,
            _ => 0,
        }
    }

    pub fn cell(i: i32) -> Self {
        match i {
            1 => Self::X,
            -1 => Self::O,
            _ => Self::N,
        }
    } 
}

/// Represents one instance of a game and holds
/// the array representation, grid of cells, 
/// and the latest and oldest turn on the grid
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct GameState {
    pub array: [u16;9],
    pub state: [[Cell;3];3],
    pub expiring: u16,
    pub last: u16,
}
impl GameState {

    pub fn new() -> Self {
        GameState::from_array([0;9])
    }

    /// parses an array of numbers received from 
    /// javascript and creates an instance.
    /// Correctly identifies latest and expiring
    /// turns
    pub fn from_array(array: [u16; 9]) -> Self {
        let mut state = Self {
            array,
            expiring: 0,
            state: [[Cell::N;3];3],
            last: 0,
        };
        let mut smallest = (300, 0, 0);
        let mut largest = 0;

        for i in 0..3 {
            for j in 0..3 {
                let index = i*3 + j;
                let num = array[index];

                state.state[i][j] = if num == 0 {
                    Cell::N
                } else if num % 2 == 0 {
                    Cell::X
                } else if num % 2 == 1 {
                    Cell::O
                } else {
                    Cell::N
                };

                if num < smallest.0 && num != 0 {
                    smallest.0 = num;
                    smallest.1 = i;
                    smallest.2 = j;
                }
                if num > largest {
                    largest = num;
                }
            }
        }
        if largest >= 6 {
            state.state[smallest.1][smallest.2] = Cell::E;
            state.expiring = smallest.0;
        }
        state.last = largest;

        state
    }

    /// accesses grid cell based on its
    /// corresponding index in the array
    pub fn get(&self, i: u16) -> Cell {
        let j = (i % 3) as usize;
        let i = (i / 3) as usize;
        self.state[i][j]
    }

    /// private function. Sets the cell at given
    /// array index to the given cell. It does
    /// NOT perform any validations, assuming it
    /// to be the calling function's responsibility
    fn set(&mut self, i: u16, cell: Cell) {
        
        let j = (i % 3) as usize;
        let i = (i / 3) as usize;
        self.state[i][j] = cell;
    }
    
    /// function used for making a move on the 
    /// grid. a move is only legal if it is on a 
    /// None Cell, otherwise it will panic.
    /// Returns the new state without modifying
    /// the current state.
    /// 
    /// The logic for moving the expiring and latest
    /// move is also included here but may be 
    /// extracted to a function of its own if needed
    pub fn turn(&self, i: u16) -> Option<Self> {
        if  Cell::N != self.get(i) {
            return None;
        }

        let mut new = self.clone();

        new.last += 1;
        let cell = if new.last % 2 == 0 {
            Cell::X
        } else {
            Cell::O
        };
        new.array[i as usize] = new.last;
        new.set(i, cell);

        if new.expiring != 0 {
            let index = new.array
            .iter()
            .enumerate()
            .find(|&x| *x.1 == new.expiring)
            .unwrap()
            .0 as u16;

            new.set(index, Cell::N);
            new.array[index as usize] = 0;
            
            new.expiring += 1;
            let index = new.array
            .iter()
            .enumerate()
            .find(|&x| *x.1 == new.expiring)
            .unwrap()
            .0 as u16;

            new.set(index, Cell::E);
        } else 
        
        if new.last == 6 {
            new.expiring += 1;
            let index = new.array
            .iter()
            .enumerate()
            .find(|&x| *x.1 == new.expiring)
            .unwrap()
            .0 as u16;

            new.set(index, Cell::E);
        }

        Some(new)
    }
}










#[cfg(test)]
mod test {
    use super::{Cell, GameState};

    #[test]
    /// Parsing a game state that has an expiring move in it
    fn game_state_correct_full() {
        let array = [
            0, 3, 0,
            4, 6, 5,
            0, 7, 2
        ];
        let expected = [
            [Cell::N, Cell::O, Cell::N],
            [Cell::X, Cell::X, Cell::O],
            [Cell::N, Cell::O, Cell::E],
        ];

        let game_state = GameState::from_array(array);
        assert_eq!(game_state.state, expected);
    }

    #[test]
    /// Parsing a game state that does not contain an 
    /// expiring move, ie, an early game state
    fn game_state_correct_partial() {
        let array = [
            0, 3, 0,
            4, 1, 5,
            0, 0, 2
        ];
        let expected = [
            [Cell::N, Cell::O, Cell::N],
            [Cell::X, Cell::O, Cell::O],
            [Cell::N, Cell::N, Cell::X],
        ];

        let game_state = GameState::from_array(array);
        assert_eq!(game_state.state, expected);
    }
    
    #[test]
    fn game_state_getter() {
        let array = [
            0, 3, 0,
            4, 6, 5,
            0, 7, 2
        ];
        
        let game_state = GameState::from_array(array);
        let got = game_state.get(4);
        let expected = Cell::X;
        assert_eq!(got, expected);
    }

    #[test]
    fn game_state_setter() {
        let array = [
            0, 3, 0,
            4, 6, 5,
            0, 7, 2
        ];
        let ex_array = [
            0, 3, 0,
            4, 0, 5,
            0, 7, 2
        ];
        let expected = GameState::from_array(ex_array);

        let mut game_state = GameState::from_array(array);
        game_state.set(4, Cell::N);

        assert_eq!(game_state.state, expected.state);
    }

    #[test]
    /// test an early game turn that has no expiring
    /// move in it or even after it
    fn game_state_turn_partial_1() {
        let array = [
            0, 0, 0,
            0, 1, 0,
            2, 0, 3,
        ];
        let ex_array = [
            0, 0, 0,
            4, 1, 0,
            2, 0, 3,
        ];

        let mut game_state = GameState::from_array(array);
        let expected = GameState::from_array(ex_array);

        game_state = game_state.turn(3).unwrap();

        assert_eq!(game_state, expected);
    }

    #[test]
    /// tests the boundary game state that itself
    /// has no expiring move but the resultant
    /// state has it, it happens exactly at turn 6
    fn game_state_turn_partial_2() {
        let array = [
            0, 5, 0,
            4, 1, 0,
            2, 0, 3,
        ];
        let ex_array = [
            0, 5, 6,
            4, 1, 0,
            2, 0, 3,
        ];

        let mut game_state = GameState::from_array(array);
        let expected = GameState::from_array(ex_array);

        game_state = game_state.turn(2).unwrap();

        assert_eq!(game_state, expected);
    }

    #[test]
    /// tests a turn right after the boundry 
    /// condition at turn 7
    fn game_state_turn_full_1() {
        let array = [
            0, 5, 0,
            4, 1, 0,
            2, 6, 3,
        ];
        let ex_array = [
            0, 5, 7,
            4, 0, 0,
            2, 6, 3,
        ];

        let mut game_state = GameState::from_array(array);
        let expected = GameState::from_array(ex_array);

        game_state = game_state.turn(2).unwrap();

        assert_eq!(game_state, expected);
    }

    #[test]
    /// tests any random turn that has no
    /// special cases
    fn game_state_turn_full_2() {
        let array = [
            0, 15, 0,
            14, 11, 0,
            12, 16, 13,
        ];
        let ex_array = [
            0, 15, 17,
            14, 0, 0,
            12, 16, 13,
        ];

        let mut game_state = GameState::from_array(array);
        let expected = GameState::from_array(ex_array);

        game_state = game_state.turn(2).unwrap();

        assert_eq!(game_state, expected);
    }

    #[test]
    /// tests the handling of an illegal turn by
    /// returning None
    fn game_state_illegal_turn() {
        let array = [
            0, 15, 0,
            14, 11, 0,
            12, 16, 13,
        ];

        let game_state = GameState::from_array(array);
        let game_state = game_state.turn(1);
        assert_eq!(game_state, None);
    }
}

/// build command: wasm-pack build --out-dir tictactoe_node\wasm

pub mod utils;
pub mod models;
pub mod ai;

use utils::{get_possibilities, state_vec_to_array};
use wasm_bindgen::prelude::*;
use models::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
/// WASM Function
/// accepts a game array and checks if
/// either of the players have won, returning 
/// the result where 1 = X, -1 = O and 
/// 0 means no one has won
pub fn check_winner(array: Vec<u16>) -> i32 {
    let array = state_vec_to_array(array);
    let game_state = GameState::from_array(array);
    let possibilities = get_possibilities();

    let mut winner = Cell::N;

    'a:for possibility in possibilities {
        let cell = game_state.get(possibility[0]);
        if cell == Cell::N || cell == Cell::E {
            continue;
        }
        for i in possibility {
            if game_state.get(i) != cell {
                continue 'a;
            }
        }

        winner = cell;
        break;
    }

    winner.number()
}

#[wasm_bindgen]
/// WASM Function
pub fn make_move_wrapper(array: Vec<u16>, tile: u16) -> Option<Vec<i32>> {
    let array = state_vec_to_array(array);
    match make_move(array, tile) {
    Some((arr, winner)) => {
        let mut vec: Vec<i32> = arr.iter().map(|&n| n as i32).collect();
        vec.push(winner);
        Some(vec)
    },
    None => None,
    }
}
/// Takes a game array and a turn request
/// the 
pub fn make_move(array: [u16; 9], tile: u16) -> Option<([u16; 9], i32)> {
    let game_state = GameState::from_array(array);

    match game_state.turn(tile) {
        Some(gs) => {
            let winner = check_winner(gs.array.to_vec());
            
            Some((gs.array, winner))
        },
        None => {
            None
        },
    }
}

#[wasm_bindgen]
///WASM Function
pub fn ai_turn_wrapper(array: Vec<u16>, tile: u16, difficulty: f32) -> Option<Vec<i32>> {
    let array = state_vec_to_array(array);
    let (new, winner) = match ai_turn(array, tile, difficulty) {
    Some(tuple) => tuple,
    None => return None,
    };

    let vec = new.to_vec();
    let mut vec: Vec<i32> = vec.iter().map(|&n| n as i32).collect();
    vec.push(winner);
    Some(vec)
}
/// Makes a turn and then lets the AI make
/// the next turn and returns the result
pub fn ai_turn(array: [u16; 9], tile: u16, difficulty: f32) -> Option<([u16; 9], i32)> {
    match make_move(array, tile) {
        Some((array, result)) => {
            if result != 0 {
                Some((array, result))
            } else {
                let game_state = GameState::from_array(array);
                let next = ai::turn(&game_state, difficulty);
                //print_values(&next.1);
                make_move(array, next)
            }
        },
        None => {
            None
        },
    }
}







#[cfg(test)]
mod test {
    use crate::{check_winner, make_move};

    #[test]
    fn verify_check_winner_x() {
        let array = vec![
            0, 15, 0,
            13, 11, 0,
            12, 16, 14,
        ];
        let result = check_winner(array);
        assert_eq!(result, 1);
    }

    #[test]
    fn verify_check_winner_o() {
        let array = vec![
            0, 17, 0,
            16, 15, 0,
            12, 13, 14,
        ];
        let result = check_winner(array);
        assert_eq!(result, -1);
    }

    #[test]
    fn verify_check_winner_none() {
        let array = vec![
            0, 15, 0,
            13, 11, 14,
            12, 16, 0,
        ];
        let result = check_winner(array);
        assert_eq!(result, 0);
    }


    #[test]
    fn verify_turn_valid() {
        let array = [
            0, 15, 0,
            13, 11, 14,
            12, 16, 0,
            ];
            let expected = [
            0, 15, 0,
            13, 0, 14,
            12, 16, 17
        ];
        let result = make_move(array, 8);
        assert_eq!(result, Some((expected, 0)));
    }

    #[test]
    fn verify_turn_invalid() {
        let array = [
            0, 15, 0,
            13, 11, 14,
            12, 16, 0,
            ];
            let expected = None;
        let result = make_move(array, 1);
        assert_eq!(result, expected);
    }
}
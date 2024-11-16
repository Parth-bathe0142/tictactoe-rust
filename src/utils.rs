use crate::GameState;

/// Helper function used for converting
/// the Vector received from javascript
/// into an array for use in rust code
pub(crate) fn state_vec_to_array(array: Vec<u16>) -> [u16; 9] {
    let mut new = [0;9];
    for (i, x) in array.iter().take(9).enumerate() {
        new[i] = *x;
    }
    new
}

/// The rows, columns and diagonals where
/// the game can be won, winning checks 
/// happen on these
pub(crate) fn get_possibilities() -> Vec<[u16; 3]> {
    vec![
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],

        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],

        [0, 4, 8],
        [2, 4, 6],
    ]
}

pub fn print_state(array: &[u16;9]) {
    let gs = GameState::from_array(array.to_owned());

    println!("|{:?}|{:?}|{:?}|", gs.get(0), gs.get(1), gs.get(2));
    println!("|{:?}|{:?}|{:?}|", gs.get(3), gs.get(4), gs.get(5));
    println!("|{:?}|{:?}|{:?}|", gs.get(6), gs.get(7), gs.get(8));
}

pub fn print_values(array: &[i32;9]) {
    println!("|{:?}|{:?}|{:?}|", array[0], array[1], array[2]);
    println!("|{:?}|{:?}|{:?}|", array[3], array[4], array[5]);
    println!("|{:?}|{:?}|{:?}|", array[6], array[7], array[8]);
}
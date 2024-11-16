use std::collections::HashMap;

use crate::utils::get_possibilities;
use crate::models::{Cell, GameState};

pub(crate) fn turn(game_state: &GameState, temperature: f32) -> u16 {

    let mut empty_tiles = Vec::new();
    let mut player = Cell::X;
    for (i, &num) in game_state.array.iter().enumerate() {
        if let Cell::N = game_state.get(i as u16) {
            empty_tiles.push(i as u16);
        } else if num == game_state.last {
            player = game_state.get(i as u16);
        }
    }

    let mut map = HashMap::new();
    for tile in empty_tiles {
        let new = game_state.turn(tile).unwrap();
        let opportunity = calculate_opportunity(&new, player, temperature);
        map.insert((opportunity * 1000.0) as i32, tile);
    }
    
    let selected= map.clone().into_keys().max().unwrap(); 

    map[&selected]
}

fn calculate_opportunity(game_state: &GameState, opponent: Cell, temperature: f32) -> f32 {
    let possibilities = get_possibilities();
    let this = if let Cell::X = opponent {
        Cell::O
    } else {
        Cell::X
    };

    let mut opportunity = (0.0, 0.0);
    for possibility in possibilities {
        
        let mut like: i32 = 0;
        let mut unlike: i32 = 0;
        for index in possibility {
            match game_state.get(index) {
                Cell::X => {
                    if let Cell::X = this {
                        like += 1;
                    } else {
                        unlike += 1;
                    }
                },
                Cell::O => {
                    if let Cell::O = this {
                        like += 1;
                    } else {
                        unlike += 1;
                    }
                },
                Cell::E => {
                    continue;
                },
                Cell::N => {},
            }
        }
        if like > 0 {
            opportunity.0 += like.pow(3) as f32;
            if like == 3 {
                opportunity.0 += 8.0;
            }
            if unlike == 2 {
                opportunity.0 += 8.0;
            }
        }



        let mut like: i32 = 0;
        let mut unlike: i32 = 0;
        for index in possibility {
            match game_state.get(index) {
                Cell::X => {
                    if let Cell::X = opponent {
                        like += 1;
                    } else {
                        unlike += 1;
                    }
                },
                Cell::O => {
                    if let Cell::O = opponent {
                        like += 1;
                    } else {
                        unlike += 1;
                    }
                },
                Cell::E => {
                    continue;
                },
                Cell::N => {},
            }
        }
        if like > 0 {
            opportunity.1 += like.pow(3) as f32;
            if like == 2 {
                opportunity.1 += 8.0;
            }
            if unlike == 2 {
                opportunity.1 += 8.0;
            }
        }
    }

    let random = if temperature > 0.0 {
        (rand::random::<f32>() * temperature * 2.0) - temperature 
    } else {0.0};


    let opportunity = (opportunity.0 - opportunity.1) + random;
    // println!("{:?} = {opportunity}", game_state);
    opportunity
}






#[cfg(test)]
mod test {
    use crate::GameState;

    use super::turn;


    #[test]
    fn ai_test_winning_1() {
        let array = [
            0, 1, 3,
            2, 0, 0,
            4, 0, 0
        ];
        let _ex_array = [
            6, 1, 3,
            2, 0, 0,
            4, 5, 0
        ];
        let game_state = GameState::from_array(array);
        let result = turn(&game_state, 0.0);

        assert_eq!(result, 0);
    }

    #[test]
    fn ai_test_winning_2() {
        let array = [
            0, 1, 3,
            2, 0, 0,
            4, 0, 5
        ];
        let _ex_array = [
            0, 0, 3,
            2, 6, 7,
            4, 0, 5
        ];
        let game_state = GameState::from_array(array);
        let result = turn(&game_state, 0.0);

        assert_eq!(result, 5);
    }
}
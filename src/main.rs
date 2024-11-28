use std::{collections::HashMap, io};
use tic_tac_toe::{ai_turn, models::{Cell, GameState}, utils::print_state};
fn main() {
    let map = HashMap::from([
        ("1", 6 as u16),
        ("2", 7),
        ("3", 8),
        ("4", 3),
        ("5", 4),
        ("6", 5),
        ("7", 0),
        ("8", 1),
        ("9", 2),
    ]);
    loop {
        println!("enter difficulty or c to exit: ");
        let mut choice = String::new();
        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        if choice.trim() == "c" {
            return;
        }
        let difficulty = choice.trim().parse::<f32>().unwrap_or(1.0);

        let mut array = GameState::new().array;

        loop {
            println!("Next turn: ");
            let mut choice = String::new();
            io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
            
            match map.get(&choice.trim()) {
                Some(&tile) => {
                    let (state, result) = match ai_turn(array, tile, difficulty) {
                        Some(tuple) => tuple,
                        None => {
                            println!("Illegal move, try again: ");
                            continue;
                        },
                    };
                    print_state(&state);

                    if result != 0 {
                        let cell = Cell::cell(result);
                        println!("{:?} Won", cell);
                        break;
                    }

                    array = state;
                },
                None => break,
            }
        }
    }
}


/* io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); */
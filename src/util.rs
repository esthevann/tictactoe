use std::{io::{stdin, stdout, Write}};

use crate::{board::{Board, Marker}, ai::Ai};

pub fn get_move(
    func: &dyn Ai,
    board: &Board,
    player: Marker,
) -> (u8, u8) {
    loop {
        let coord = func.get_move(board, player);
        if board.is_valid_move(coord) {
            break coord;
        }
        println!();
    }
}

fn prompt(txt: &str) -> String {
    let mut name = String::new();
    print!("{txt}");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut name)
        .expect("Couldn't read user input");
    name.trim().to_string()
}

pub struct HumanAi;
impl Ai for HumanAi {
    fn get_move(&self, _board: &Board, _player: Marker) -> (u8, u8) {
        let input_x = prompt("What is your move's X co-ordinate?: ");
        let input_y = prompt("What is your move's Y co-ordinate?: ");
    
        (
            input_x
                .parse()
                .expect("Couldn't parse user input as number"),
            input_y
                .parse()
                .expect("Couldn't parse user input as number"),
        )
    }
}



pub fn get_opponent(player: Marker) -> Marker {
    match player {
        Marker::X => Marker::O,
        Marker::O => Marker::X,
    }
}

pub fn menu() -> u8 {
    println!("1 - human vs human");
    println!("2 - human vs ai");
    println!("3 - ai vs ai");
    get_option_input()
}

fn get_option_input() -> u8 {
    match prompt("Choose your game 1-3: ").parse::<u8>() {
        Ok(i) => {
            if let 1..=3 = i {
                return i;
            } else {
                get_option_input()
            }
        },
        Err(_) => get_option_input(),
    }
}
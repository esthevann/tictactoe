#![feature(let_chains)]


use crate::ai::Ai;
use ai::{MinimaxAi};
use board::{Board, Marker};

use util::{get_move, menu, HumanAi};



mod board;
mod util;
mod ai;

fn main() {
    let mut board = Board::new();

    let mut player = true;

    let option = menu();
    let (x, o): (&dyn Ai, &dyn Ai) = match option {
        1 => (&HumanAi, &HumanAi),
        2 => (&HumanAi, &MinimaxAi),
        3 => (&MinimaxAi, &MinimaxAi),
        _ => unreachable!()
    };

    board.render();
    loop {
            let coord = if player {
                    
                    get_move(x, &board, Marker::O)
                    
                } else {
                    get_move(o, &board, Marker::X)
                };
        board.make_move(
            coord,
            if player {
                Marker::O
            } else {
                Marker::X
            },
        );
        player = !player;
        board.render();
        //println!("{}", minimax_score(&board, Marker::X));
        if let Some(w) = board.get_winner() {
            println!("\n{w} player wins");
            break;
        }
        if board.is_board_full() {
            println!("\n it's a draw");
            break;
        }
    }
}


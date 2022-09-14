use crate::{
    board::{Board, Marker},
    util::get_opponent,
};
use rand::prelude::*;

pub trait Ai {
    fn get_move(&self, board: &Board, player: Marker) -> (u8, u8);
}

pub struct RandomAi;

impl Ai for RandomAi {
    fn get_move(&self, board: &Board, _player: Marker) -> (u8, u8) {
        let mut rng = thread_rng();
        loop {
            let row: u8 = rng.gen_range(0..3);
            let column: u8 = rng.gen_range(0..3);
    
            if board.is_valid_move((row, column)) {
                break (row, column);
            }
        }
    }
}


// pub struct FindsWinningAndLosingMovesAi;
// impl Ai for FindsWinningAndLosingMovesAi {
//     fn get_move(&self, board: &Board, player: Marker) -> (u8, u8) {
//         if let Some(w_move) = check_for_decisive_move(board, player) {
//             return w_move;
//         } else if let Some(w_move) = check_for_decisive_move(board, get_opponent(player)) {
//             return w_move;
//         }
    
//         RandomAi.get_move(board, player)
//     }
// }

// pub fn finds_winning_and_losing_moves_ai(board: &Board, player: Marker) -> (u8, u8) {
//     if let Some(w_move) = check_for_decisive_move(board, player) {
//         return w_move;
//     } else if let Some(w_move) = check_for_decisive_move(board, get_opponent(player)) {
//         return w_move;
//     }

//     RandomAi.get_move(board, player)
// }

// fn check_for_decisive_move(board: &Board, player: Marker) -> Option<(u8, u8)> {
//     for (row_i, row) in board.board.iter().enumerate() {
//         for (item_i, _item) in row.iter().enumerate() {
//             if !board.is_valid_move((row_i as u8, item_i as u8)) {
//                 continue;
//             }
//             let mut new_board = Board::new();
//             new_board.board = board.board;
//             new_board.make_move((row_i as u8, item_i as u8), player);
//             if new_board.get_winner().is_some() {
//                 return Some((row_i as u8, item_i as u8));
//             }
//         }
//     }
//     None
// }

pub fn minimax_score(board: &Board, player: Marker, opp: Marker) -> i8 {
    if let Some(winner) = board.get_winner()  {
        if winner == opp {
            return 10;
        } else {
            return -10;
        }
    }

    if board.is_board_full() {
        return 0;
    }

    let legal_moves = board.get_legal_moves();

    if legal_moves.is_empty() {
        return 0;
    }

    let mut scores: Vec<i8> = Vec::new();
    for move_ in legal_moves {
        let mut new_board = Board::new();
        new_board.board = board.board;
        new_board.make_move(move_, player);
        let score = minimax_score(&new_board, get_opponent(player), opp);
        scores.push(score);
    }

    if player == opp {
        *scores.iter().max().expect("vec isn't empty")
    } else {
        *scores.iter().min().expect("vec isn't empty")
    }
}

pub struct MinimaxAi;
impl Ai for MinimaxAi {
    fn get_move(&self, board: &Board, player: Marker) -> (u8, u8) {
        let mut best_move: Option<(u8, u8)> = None;
        let mut best_score: Option<i8> = None;
        for i in board.get_legal_moves() {
            let mut new_board = Board::new();
            new_board.board = board.board;
            new_board.make_move(i, player);
            let score = minimax_score(&new_board, get_opponent(player), player);
            if best_score.is_none() || score > best_score.unwrap() {
                best_move = Some(i);
                best_score = Some(score);
            }
        }
        best_move.unwrap()
    }
}


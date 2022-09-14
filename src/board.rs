
pub struct Board {
    pub board: [[Option<Marker>; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn render(&self) {
        println!("   0 1 2");
        println!("   -----");
        for (i, row) in self.board.iter().enumerate() {
            print!("{i} |");
            let mut iter = row.iter().peekable();
            while let Some(marker) = iter.next() {
                if let Some(m) = marker {
                    if iter.peek().is_none() {
                        print!("{m}");
                    } else {
                        print!("{m} ");
                    }
                } else if iter.peek().is_none() {
                        print!(" ");
                
                } else {
                        print!("  ");
                }
            }
            print!("|");
            println!();
        }
    }

    pub fn make_move(&mut self, coord: (u8, u8), player: Marker) {
        self.board[coord.0 as usize][coord.1 as usize] = Some(player);
    }

    pub fn is_valid_move(&self, coord: (u8, u8)) -> bool {
        if coord.0 > 3 || coord.1 > 3 {
            return false;
        }

        if let Some(a) = self.board.get(coord.0 as usize) 
            && let Some(u) = a.get(coord.1 as usize)
            && u.is_none() {
                return true;
        }

        false
    }

    pub fn get_legal_moves(&self) -> Vec<(u8, u8)> {
        let mut all_legal_moves = Vec::new();
        for (row_i, row) in self.board.iter().enumerate() {
            let moves = row.iter().enumerate().filter(|(_, item)| item.is_none()).map(|(i, _)| (row_i as u8, i as u8));
            all_legal_moves.extend(moves);
        }
        all_legal_moves
    }

    pub fn is_board_full(&self) -> bool {
        self.board.iter().all(|x| x.iter().all(|x| x.is_some()))
    }

    pub fn get_winner(&self) -> Option<Marker> {
        let diagonal_1 = &[self.board[0][0], self.board[1][1], self.board[2][2]];
        let diagonal_2 = &[self.board[0][2], self.board[1][1], self.board[2][0]];

        let rows = 
            // rows and diagonals
            [&self.board[0], &self.board[1],
            &self.board[2], diagonal_1, diagonal_2,

            // verticals
            &[self.board[0][0], self.board[1][0],
            self.board[2][0]],

             &[self.board[0][1],
            self.board[1][1], self.board[2][1]]
            ,
            &[self.board[0][2], self.board[1][2],
            self.board[2][2]]
            ];

        for row in rows.into_iter() {
            if row.iter().all(|x| x.is_some()) {
                if let Some(w) = Self::check_row(row) {
                    return Some(w); 
                }
            }
        }

        None
    }

    fn check_row(row: &[Option<Marker>; 3]) -> Option<Marker> {
        if row.iter().all(|x| x.unwrap() == Marker::O) {
            Some(Marker::O)
        } else if row.iter().all(|x| x.unwrap() == Marker::X) {
            Some(Marker::X)
        } else {
            None
        }
    }

}

#[derive(PartialEq, Eq,Debug, Copy, Clone)]
pub enum Marker {
    X,
    O,
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Marker::X => write!(f, "X"),
            Marker::O => write!(f, "O"),
        }
    }
}


#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White
}

// pub type Position(i8, i8);

pub enum AN {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

#[derive(Copy, Clone)]
pub enum Square {
    Empty,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color)
}

pub struct Board {
    pub squares: [Square; 64],
}

impl Board {
    pub fn init(&mut self) {
        for index in 0..63 {
            self.squares[index] = Square::Empty
        }

        //self.squares[Board::ind(AN::A, 1)] =  Square::Rook(Color::White);
        self.squares[0] =  Square::Rook(Color::White);
        self.squares[1] =  Square::Bishop(Color::White)
    }

    fn ind(row: AN, col: i32) -> i32
    {
        (row as i32) * 8 + (col - 1)
    }
}

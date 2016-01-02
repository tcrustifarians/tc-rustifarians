
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
    pub fn new() -> Self {
        Board {
            squares: [Square::Empty; 64] 
        }
    }

    pub fn init(&mut self) {
        // get rid of unused warnings
        self.squares[Self::ind(AN::C, 1)] =  Square::Empty;
        self.squares[Self::ind(AN::D, 1)] =  Square::Empty;
        self.squares[Self::ind(AN::E, 1)] =  Square::Empty;
        self.squares[Self::ind(AN::F, 1)] =  Square::Empty;

        //self.squares[Board::ind(AN::A, 1)] =  Square::Rook(Color::White);
        self.squares[Self::ind(AN::A, 1)] =  Square::Rook(Color::White);
        self.squares[Self::ind(AN::A, 2)] =  Square::Knight(Color::White);
        self.squares[Self::ind(AN::A, 3)] =  Square::Bishop(Color::White);
        self.squares[Self::ind(AN::A, 4)] =  Square::Queen(Color::White);
        self.squares[Self::ind(AN::A, 5)] =  Square::King(Color::White);
        self.squares[Self::ind(AN::A, 6)] =  Square::Bishop(Color::White);
        self.squares[Self::ind(AN::A, 7)] =  Square::Knight(Color::White);
        self.squares[Self::ind(AN::A, 8)] =  Square::Rook(Color::White);

        self.squares[Self::ind(AN::H, 1)] =  Square::Rook(Color::Black);
        self.squares[Self::ind(AN::H, 2)] =  Square::Knight(Color::Black);
        self.squares[Self::ind(AN::H, 3)] =  Square::Bishop(Color::Black);
        self.squares[Self::ind(AN::H, 4)] =  Square::Queen(Color::Black);
        self.squares[Self::ind(AN::H, 5)] =  Square::King(Color::Black);
        self.squares[Self::ind(AN::H, 6)] =  Square::Bishop(Color::Black);
        self.squares[Self::ind(AN::H, 7)] =  Square::Knight(Color::Black);
        self.squares[Self::ind(AN::H, 8)] =  Square::Rook(Color::Black);

        for index in 1..8 {
            self.squares[Self::ind(AN::B, index)] =  Square::Pawn(Color::White);
            self.squares[Self::ind(AN::G, index)] =  Square::Pawn(Color::Black);
        }
    }

    fn ind(row: AN, col: i32) -> usize
    {
        ((row as i32) * 8 + (col - 1)) as usize
    }
}

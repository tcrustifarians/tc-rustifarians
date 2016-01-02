mod board;

use board::Board;
use board::Square;

fn main() {
    let mut board: Board = Board {
        squares: [Square::Empty; 64] 
    };

    board.init();
}

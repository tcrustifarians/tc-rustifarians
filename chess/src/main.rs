mod board;

use board::Board;
use board::Square;

fn main() {
    let mut board: Board = Board::new();

    board.init();
}

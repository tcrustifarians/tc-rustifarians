mod board;

use board::Board;

fn main() {
    let mut board: Board = Board::new();

    board.init();
}

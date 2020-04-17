mod lib;

fn main() {
    let mut board = lib::connect4::Connect4Board::new();
    board.drop(1,1);
    board.drop(1,1);
    board.drop(1,1);
    board.drop(1,1);
    board.display();
    
}

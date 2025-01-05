mod gameboard;
mod helper;
use gameboard::GameBoard;
fn main() {
    let mut game: GameBoard = GameBoard::new();
    if let Err(_) = game.start() {
        println!("Game exited unexpectedly,please restart")
    }
}

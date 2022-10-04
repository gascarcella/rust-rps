mod game;

use game::*;

fn main() {
    let max_rounds: i8 = 3;
    let mut game = Game::new(
        PlayerType::Human,
        PlayerType::Computer,
        max_rounds,
    );
    println!("{:?}", game);
    game.start();
}

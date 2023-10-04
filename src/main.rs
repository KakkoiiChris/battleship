mod game;
mod tile;

use crate::game::Game;

fn main() {
    let mut game = Game::new();

    game.play();
}
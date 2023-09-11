mod game;
mod tile;

use crate::game::Game;

fn main() {
    let game = Game::new();

    game.play();
}
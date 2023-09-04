mod game;
mod tile;

use rand::{random, Rng};

fn main() {
    let x = rand::thread_rng().gen_range(0..=100);

    println!("{x}");
}
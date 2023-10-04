use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};

use rand::Rng;

use crate::tile::Tile;

pub struct Game {
    board: [[Tile; 8]; 8],
    torpedoes: u8,
    reveals: u8,
}

impl Game {
    pub fn new() -> Game {
        let mut board = [[Tile::new(false); 8]; 8];

        let a = rand::thread_rng().gen_range(0..8);
        let b = rand::thread_rng().gen_range(0..5);
        let vertical = rand::random::<bool>();

        for i in 0..4 {
            if vertical {
                board[b + i][a].set_as_ship();
            } else {
                board[a][b + i].set_as_ship();
            }
        }

        Game {
            board,
            torpedoes: 20,
            reveals: 0,
        }
    }

    pub fn play(&mut self) {
        println!("<[BATTLESHIP]>");

        loop {
            println!("{self}");

            let row = get_coord("Row");
            let col = get_coord("Column");

            self.board[row][col].reveal();

            if self.board[row][col].is_ship() {
                self.reveals += 1;
            }

            if self.reveals == 4 { break; }

            self.torpedoes -= 1;

            if self.torpedoes == 0 { break; }
        }
    }
}

fn get_coord(name: &str) -> usize {
    loop {
        print!("{name} > ");

        let _ = stdout().flush();

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {}

            Err(e) => {
                println!("Input Error: {}", e);
                continue;
            }
        }

        match input.trim().parse::<usize>() {
            Ok(num) => break num,

            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let header = (0..8)
            .collect::<Vec<i32>>()
            .iter()
            .map(|i| format!("{i}"))
            .collect::<Vec<String>>()
            .join(" ");

        writeln!(f, "  {header}")?;

        for r in 0..self.board.len() {
            let label = match std::char::from_u32((r as u32) + 65) {
                None => ' ',
                Some(c) => c,
            };

            let tiles = self.board[r]
                .iter()
                .map(|t| format!("{t}"))
                .collect::<Vec<String>>()
                .join(" ");

            writeln!(f, "{label} {tiles}")?;
        }

        writeln!(f, "TORPEDOES: {}", self.torpedoes)
    }
}
use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::tile::Tile;

pub struct Game {
    board: [[Tile; 8]; 8],
    torpedoes: u8,
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
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let header = (0..8)
            .collect::<Vec<i32>>()
            .iter()
            .map(|i| format!("{i} "))
            .collect::<Vec<String>>()
            .join(" ");

        writeln!(f, "  {header}")?;

        for r in 0..self.board.len() {
            writeln!(f, "{} {}", (r as char) + 'A', self.board[r].join(" "))?;
        }

        Ok(())
    }
}
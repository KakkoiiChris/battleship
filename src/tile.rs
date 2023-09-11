use std::fmt::{Display, Formatter};

const WATER: char = '~';
const SHIP: char = 'S';
const MISS: char = '*';

#[derive(Copy, Clone)]
pub struct Tile {
    is_ship: bool,
    is_hidden: bool,
}

impl Tile {
    pub fn new(is_ship: bool) -> Tile {
        Tile {
            is_ship,
            is_hidden: true,
        }
    }

    pub fn set_as_ship(&mut self) {
        self.is_ship = true;
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = if self.is_hidden {
            WATER
        } else if self.is_ship {
            SHIP
        } else {
            MISS
        };

        write!(f, "{symbol}")
    }
}
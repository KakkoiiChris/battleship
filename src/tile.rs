#[derive(Copy, Clone)]
pub struct Tile {
    is_ship: bool,
    is_revealed: bool,
}

impl Tile {
    pub fn new(is_ship: bool) -> Tile {
        Tile {
            is_ship,
            is_revealed: false,
        }
    }

    pub fn set_as_ship(&mut self) {
        self.is_ship = true;
    }
}
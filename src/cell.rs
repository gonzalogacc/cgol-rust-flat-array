use rand::Rng;

#[derive(Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new_random(board_size: usize) -> Cell {
        Cell {
            x: rand::rng().random_range(0..board_size as usize),
            y: rand::rng().random_range(0..board_size as usize),
        }
    }

    pub fn from_flat_index(flat_index: usize, board_size: usize) -> Cell {
        let y_index = flat_index / board_size;
        let x_index = flat_index % board_size;
        Cell {
            y: y_index,
            x: x_index,
        }
    }

    pub fn flat_index(&self, board_size: usize) -> usize {
        // Compute the flat coordinates for a cell
        self.x + (self.y * board_size)
    }


}
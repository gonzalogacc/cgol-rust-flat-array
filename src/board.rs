use crate::Cell;

pub struct Board {
    board_size: usize,
    board: Vec<bool>,
}

impl Board {
    pub fn new(board_size: usize) -> Board {
        Board {
            board_size,
            board: vec![false; board_size * board_size],
        }
    }

    pub fn set_cell(&mut self, cell: &Cell, value: bool) {
        self.board[cell.y * self.board_size + cell.x] = value
    }

    fn get_cell_neighbours(&self, cell: &Cell) -> i32 {
        // Compute the number of neighbours for a cell
        let mut nn_count = 0;
        for y in -1_isize..=1_isize {
            for x in -1_isize..=1_isize {
                if x == y && x == 0 {
                    continue;
                }

                // Calculate the neighbor coordinates
                let x_nn = cell.x as isize + x;
                let y_nn = cell.y as isize + y;

                // Check if the neighbor is within the board boundaries
                if x_nn >= 0
                    && x_nn < self.board_size as isize
                    && y_nn >= 0
                    && y_nn < self.board_size as isize
                {
                    // If within bounds, calculate the flat index and check the cell
                    let flat_index = (y_nn * self.board_size as isize + x_nn) as usize;
                    if self.board[flat_index] {
                        nn_count += 1;
                    }
                }
            }
        }
        nn_count
    }

    fn apply_rules(&self, cell: &Cell) -> bool {
        let nn = self.get_cell_neighbours(&cell);
        let cell_status = self.board[cell.flat_index(self.board_size)];

        let cell_result = match nn {
            nn if cell_status && nn < 2 => false,
            nn if cell_status && nn >= 2 && nn <= 3 => true,
            nn if !cell_status && nn == 3 => true,
            _ => false,
        };
        cell_result
    }

    pub fn print(&self) {
        let mut final_string = String::new();
        for y in 0..self.board_size {
            for x in 0..self.board_size {
                let coord = (y * self.board_size + x) as usize;
                let nchar = match self.board[coord] {
                    true => 'O',
                    false => '.',
                };
                final_string.push(nchar);
            }
            final_string.push('\n');
        }
        println!("{}", final_string);
    }

    pub fn run_step(&mut self, temp_vec: &mut Vec<bool>) {
        for p in 0..self.board.len() {
            let y_index = p / self.board_size;
            let x_index = p % self.board_size;
            let cell_status = self.apply_rules(&Cell {
                x: x_index,
                y: y_index,
            });
            temp_vec[p] = cell_status;
        }
        self.board = temp_vec.clone();
    }
}

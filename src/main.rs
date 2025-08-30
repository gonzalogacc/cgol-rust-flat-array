mod board;
mod cell;

use crate::board::Board;
use crate::cell::Cell;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::{thread, time::Duration};
use std::path::PathBuf;
use clap::Parser;

fn load_pattern(file_name: PathBuf) -> Vec<Cell> {
    // Load pattern into a
    let mut pattern: Vec<Cell> = vec![];

    let file_content = read_to_string(file_name).expect("Could not open file");
    for (y_index, line) in file_content.lines().enumerate() {
        for (x_index, c) in line.chars().enumerate() {
            if c == 'O' {
                pattern.push(Cell {
                    x: x_index,
                    y: y_index,
                });
            }
        }
    }
    pattern
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value="20")]
    board_size: usize,

    #[arg(short, long)]
    pattern_file: PathBuf,

    #[arg(short, long, default_value="100")]
    steps: usize,
}


fn main() {
    // Create a flat vector to hold the board
    let args = Cli::parse();

    let board_size = args.board_size;
    let pattern = load_pattern(args.pattern_file);
    let steps = args.steps;

    println!("{:?}", pattern);

    let mut temp_vec = vec![false; board_size * board_size];
    let mut board = Board::new(board_size);
    println!("Initial state:");

    for cell in pattern.iter() {
        board.set_cell(cell, true);
    }

    for _ in 1..steps {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
        board.run_step(&mut temp_vec);
        board.print();
        thread::sleep(Duration::from_millis(100));
    }
}

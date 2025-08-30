# Conway's Game of Life

This is a command-line implementation of Conway's Game of Life, a fascinating cellular automaton devised by the mathematician John Horton Conway in 1970.

The simulation runs for a specified number of steps, displaying the board's evolution directly in the terminal. The initial pattern is loaded from a user-provided text file, allowing you to easily test different starting configurations.

## Features

* **File-Based Patterns**: Load complex starting configurations from a simple text file.

* **Customizable Board**: Set the board size and the number of simulation steps via command-line arguments.

* **Console Animation**: Watch the simulation unfold with a clean, step-by-step display.

## Prerequisites

You'll need to have Rust and Cargo installed on your system. If you don't have them, you can install them from the official [Rust website](https://www.rust-lang.org/tools/install).

## Usage

1. **Clone the Repository**

   Navigate to your desired directory and clone the project:

git clone <your-repo-url>
cd rust-game-of-life


(Remember to replace `<your-repo-url>` with the actual URL of your repository.)

2. **Create or Find a Pattern File**

The program uses a plain text format to load patterns. You can either create your own or find existing ones from the Game of Life community. An `O` represents a living cell, while any other character (such as a dot or a space) is an empty cell. The dimensions of the pattern are inferred from the file.

Example: `glider.txt`

.O.
..O
OOO


You can find a vast collection of pre-made patterns from the [Conway's Game of Life Wiki](https://conwaylife.com/wiki/Main_Page). Look for patterns in their plain text (`.cells`) format, which is directly compatible with this program. Simply copy the pattern and paste it into a local `.txt` file.

3. **Run the Simulation**

Use the `cargo run` command with the appropriate arguments.

Run with a 20x20 board, using the 'glider.txt' pattern, for 100 steps
cargo run -- -b 20 -p glider.txt -s 100


## Command-line Arguments

* `-b, --board-size <SIZE>`: Sets the board size. The default value is `20`.

* `-p, --pattern-file <FILE_PATH>`: **(Required)** Specifies the path to the pattern file.

* `-s, --steps <COUNT>`: Sets the number of simulation steps. The default value is `100`.

## Building from Source

To build a standalone executable that you can run without `cargo`, use the following command:

cargo build --release


The executable will be located at `target/release/rust-game-of-life`.
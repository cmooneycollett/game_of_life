# Game of Life

A Rust-based version of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) run from the command-line.

## Pre-requisites

1. Rust 1.40.0 installed.
2. Bash shell environment.

The program currently has no dependencies outside of the Rust standard library.

## Compilation

The program can be compiled using the `cargo` utility using:

```bash
cargo build --release
```

## Usage

The program takes command-line input in the following form:

```bash
./game_of_life <input_file> <time per step (ms)>
```

## Input file format

The input files for the program take the following form.

1. Line 1 consists of dimensions of the field, in form `<width>,<height>`
2. Subsequent number of lines of expected length and quantity consist of comma-separated values representing the initial state of the square. 0 represents a dead square and 1 represents an alive square.

An example input file for a "blinker" is as follows:

```text
5,5
0,0,0,0,0
0,0,0,0,0
0,1,1,1,0
0,0,0,0,0
0,0,0,0,0
```

Some example input files are included in the `./inputs/` folder.

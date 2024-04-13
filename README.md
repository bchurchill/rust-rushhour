# RushHour Solver in Rust

## Overview
This repository demonstrates a Rust-based solver for the RushHour logic puzzle, created for educational purposes. RushHour is a popular sliding block puzzle that involves moving a set of obstructing vehicles out of the way to clear a path for the red car to exit the grid. For a hands-on experience of the game, you can play it online [here](https://www.thinkfun.com/rush-hour-online-play/).

## How It Works
The solver uses a breadth-first search algorithm to explore all possible movements of vehicles on the board until it finds a winning position. Each state of the board represents a node in the search space. By navigating through these nodes, the solver finds the shortest sequence of moves that leads to the red car's escape.

## Usage
To run the solver, simply clone this repository and build the project using Cargo:

```bash
cargo build --release
cargo run --release


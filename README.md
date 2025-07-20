# Conway's Game of Life in Rust

A simple implementation of Conway's Game of Life written in Rust using the Raylib library.

![Conway's Game of Life Demo](game_of_life_demo.gif)

## About The Project

This project simulates Conway's Game of Life, a zero-player game determined by its initial state. The simulation is rendered in a Raylib window, and the final state of the grid is saved as a PNG image upon closing the application.

## Features

- Real-time simulation of cellular automata.
- Implemented with a clean, modular structure.
- Renders the grid to a texture for efficient drawing.
- Saves the final game state to `final_game_state.png`.

## How to Run

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/your-username/game_of_life.git
    cd game_of_life
    ```

2.  **Run the project:**
    ```sh
    cargo run --release
    ```

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [Raylib-rs](https://github.com/deltaphc/raylib-rs)

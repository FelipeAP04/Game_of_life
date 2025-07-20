use raylib::prelude::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

pub struct GameOfLifeGrid {
    width: i32,
    height: i32,
    cells: Vec<CellState>,
    live_color: Color,
    dead_color: Color,
}

impl GameOfLifeGrid {
    pub fn new(width: i32, height: i32, live_color: Color, dead_color: Color) -> Self {
        GameOfLifeGrid {
            width,
            height,
            cells: vec![CellState::Dead; (width * height) as usize],
            live_color,
            dead_color,
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<CellState> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.cells[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, state: CellState) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.cells[(y * self.width + x) as usize] = state;
        }
    }

    pub fn get_color(&self, state: CellState) -> Color {
        match state {
            CellState::Alive => self.live_color,
            CellState::Dead => self.dead_color,
        }
    }

    fn count_live_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut live_neighbors = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x + dx;
                let ny = y + dy;

                let wrapped_nx = (nx + self.width) % self.width;
                let wrapped_ny = (ny + self.height) % self.height;

                if let Some(CellState::Alive) = self.get_cell(wrapped_nx, wrapped_ny) {
                    live_neighbors += 1;
                }
            }
        }
        live_neighbors
    }

    pub fn next_generation(&mut self) {
        let mut next_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let current_cell_state = self.get_cell(x, y).unwrap();
                let live_neighbors = self.count_live_neighbors(x, y);
                let index = (y * self.width + x) as usize;

                next_cells[index] = match current_cell_state {
                    CellState::Alive => {
                        if live_neighbors < 2 || live_neighbors > 3 {
                            CellState::Dead
                        } else {
                            CellState::Alive
                        }
                    }
                    CellState::Dead => {
                        if live_neighbors == 3 {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        }
                    }
                };
            }
        }
        self.cells = next_cells;
    }

    pub fn apply_pattern(&mut self, pattern_points: &[(i32, i32)]) {
        for &(x, y) in pattern_points {
            self.set_cell(x, y, CellState::Alive);
        }
    }
}

pub mod patterns {
    pub fn glider(offset_x: i32, offset_y: i32) -> Vec<(i32, i32)> {
        vec![
            (1 + offset_x, 0 + offset_y),
            (2 + offset_x, 1 + offset_y),
            (0 + offset_x, 2 + offset_y),
            (1 + offset_x, 2 + offset_y),
            (2 + offset_x, 2 + offset_y),
        ]
    }

    pub fn small_exploder(offset_x: i32, offset_y: i32) -> Vec<(i32, i32)> {
        vec![
            (1 + offset_x, 0 + offset_y),
            (0 + offset_x, 1 + offset_y),
            (1 + offset_x, 1 + offset_y),
            (2 + offset_x, 1 + offset_y),
            (0 + offset_x, 2 + offset_y),
            (2 + offset_x, 2 + offset_y),
            (1 + offset_x, 3 + offset_y),
        ]
    }

    pub fn ten_cell_row(offset_x: i32, offset_y: i32) -> Vec<(i32, i32)> {
        vec![
            (0 + offset_x, 0 + offset_y),
            (1 + offset_x, 0 + offset_y),
            (2 + offset_x, 0 + offset_y),
            (3 + offset_x, 0 + offset_y),
            (4 + offset_x, 0 + offset_y),
            (5 + offset_x, 0 + offset_y),
            (6 + offset_x, 0 + offset_y),
            (7 + offset_x, 0 + offset_y),
            (8 + offset_x, 0 + offset_y),
            (9 + offset_x, 0 + offset_y),
        ]
    }

    pub fn r_pentomino(offset_x: i32, offset_y: i32) -> Vec<(i32, i32)> {
        vec![
            (1 + offset_x, 0 + offset_y),
            (2 + offset_x, 0 + offset_y),
            (0 + offset_x, 1 + offset_y),
            (1 + offset_x, 1 + offset_y),
            (1 + offset_x, 2 + offset_y),
        ]
    }

    pub fn blinker(offset_x: i32, offset_y: i32) -> Vec<(i32, i32)> {
        vec![
            (0 + offset_x, 0 + offset_y),
            (1 + offset_x, 0 + offset_y),
            (2 + offset_x, 0 + offset_y),
        ]
    }
}
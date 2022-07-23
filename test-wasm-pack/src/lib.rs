mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

extern crate js_sys;

fn random_bool() -> bool {
    js_sys::Math::random() < 0.5
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn make_cells(&self) -> Vec<Cell> {
        (0..self.width * self.height).map(|_i| Cell::Dead).collect()
    }
}

#[wasm_bindgen]
pub enum CellMotif {
    Lines,
    Spaceship,
    Random,
}

fn make_cells(width: u32, height: u32, motif: CellMotif) -> Vec<Cell> {
    let ranger = 0..width * height;
    match motif {
        CellMotif::Lines => ranger
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect(),
        CellMotif::Spaceship => {
            let mut cells: Vec<Cell> = ranger.map(|_i| Cell::Dead).collect();
            for i in [0, width + 1, width + 2, 2 * width, 2 * width + 1] {
                cells[i as usize] = Cell::Alive;
            }
            cells
        }
        CellMotif::Random => ranger
            .map(|_i| {
                if random_bool() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect(),
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(motif: CellMotif) -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = make_cells(width, height, motif);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = self.make_cells();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = self.make_cells();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        let mut new_living_cells: Vec<(u32, u32)> = Vec::new();
        let mut new_dead_cells: Vec<(u32, u32)> = Vec::new();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, column);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    column,
                    cell,
                    live_neighbors
                );

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                log!("   it becomes {:?}", next_cell);

                match (cell != next_cell, next_cell) {
                    (true, Cell::Alive) => new_living_cells.push((row, column)),
                    (true, Cell::Dead) => new_dead_cells.push((row, column)),
                    _ => {}
                }

                next[idx] = next_cell;
            }
        }

        log!(
            "new living cells ({}): {:?}",
            new_living_cells.len(),
            new_living_cells
        );
        log!(
            "new dead cells ({}): {:?}",
            new_dead_cells.len(),
            new_dead_cells
        );
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, column) in cells.iter().cloned() {
            let idx = self.get_index(row, column);
            self.cells[idx] = Cell::Alive;
        }
    }
}

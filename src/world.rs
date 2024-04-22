use rand::{thread_rng, Rng};

use crate::cell::Cell;

/// Represents a world that contains a 3D grid of dimension 'size'.
pub struct World {
    pub grid: Vec<Vec<Vec<Cell>>>,  // essentially a 3D array (a vector of vectors of vectors)
    pub size: [usize; 3]            // the dimensions of the world grid
}

impl World {
    /// Creates a new instance of 'World' with some preset properties.
    pub fn new(size: [usize; 3]) -> Self {
        let grid = vec![vec![vec![Cell {
            position: [0, 0, 0],
            is_alive: false,
            color: [1.0, 1.0, 1.0]
        }; size[2]]; size[1]]; size[0]];

        World { grid, size }
    }

    /// Initalizes the world with a 3D grid of cells.
    pub fn initialize(&mut self) {
        for x in 0..self.size[0] {
            for y in 0..self.size[1] {
                for z in 0..self.size[2] {
                    self.grid[x][y][z] = Cell {
                        position: [x as i32, y as i32, z as i32],
                        is_alive: thread_rng().gen_bool(1.0 / 100.0),
                        color: if rand::random() { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] }
                    };
                }
            }
        }
    }

    /// Updates the world state (currently unused).
    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone(); // probably inefficient
        
        for x in 0..self.size[0] {
            for y in 0..self.size[1] {
                for z in 0..self.size[2] {
                    let alive_neighbors = self.count_alive_neighbors(x, y, z);
                    let cell = &self.grid[x][y][z];
                    let new_state = match (cell.is_alive, alive_neighbors) {
                        (true, 2..=3) | (false, 3) => true,
                        _ => false,
                    };

                    new_grid[x][y][z].is_alive = new_state;
                    new_grid[x][y][z].color = if new_state { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
                }
            }
        }

        self.grid = new_grid;
    }

    /// Count the alive neighbors of a cell (currently unimplemented).
    pub fn count_alive_neighbors(&self, x: usize, y: usize, z: usize) -> usize {
        0 // placeholder, need to implement neighbor logic
    }
}
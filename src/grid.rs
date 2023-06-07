use std::os::raw::c_int;

use raylib::ffi::{Color, DrawRectangle};

use crate::colors::get_cell_colors;

#[derive(Debug, Default)]
pub struct Grid {
    pub grid: [[c_int; 10]; 20],
    num_rows: c_int,
    num_cols: c_int,
    cell_size: c_int,
    colors: Vec<Color>,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            num_rows: 20,
            num_cols: 10,
            cell_size: 30,
            ..Default::default()
        };

        grid.initialize();
        grid.colors = get_cell_colors();

        grid
    }

    pub fn initialize(&mut self) {
        for row in 0..self.num_rows as usize {
            for colomn in 0..self.num_cols as usize {
                self.grid[row][colomn] = 0;
            }
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for row in 0..self.num_rows as usize {
            for column in 0..self.num_cols as usize {
                print!("{} ", self.grid[row][column]);
            }
            println!();
        }
    }

    pub fn draw(&self) {
        for row in 0..self.num_rows {
            for column in 0..self.num_cols {
                let cell_value = self.grid[row as usize][column as usize];
                unsafe {
                    DrawRectangle(
                        column * self.cell_size + 11,
                        row * self.cell_size + 11,
                        self.cell_size - 1,
                        self.cell_size - 1,
                        self.colors[cell_value as usize],
                    );
                }
            }
        }
    }

    pub fn is_cell_outside(&self, row: c_int, column: c_int) -> bool {
        if row >= 0 && row < self.num_rows && column >= 0 && column < self.num_cols {
            return false;
        }

        true
    }

    pub fn is_cell_empty(&self, row: c_int, column: c_int) -> bool {
        if self.grid[row as usize][column as usize] == 0 {
            return true;
        }

        false
    }

    pub fn clear_full_rows(&mut self) -> c_int {
        let mut completed = 0;

        for row in (0..=self.num_rows - 1).rev() {
            if self.is_row_full(row) {
                self.clear_row(row);
                completed += 1;
            } else if completed > 0 {
                self.move_row_down(row, completed);
            }
        }

        completed
    }

    fn is_row_full(&self, row: c_int) -> bool {
        for column in 0..self.num_cols {
            if self.grid[row as usize][column as usize] == 0 {
                return false;
            }
        }

        true
    }

    fn clear_row(&mut self, row: c_int) {
        for column in 0..self.num_cols {
            self.grid[row as usize][column as usize] = 0;
        }
    }

    fn move_row_down(&mut self, row: c_int, num_rows: c_int) {
        for column in 0..self.num_cols {
            self.grid[(row + num_rows) as usize][column as usize] =
                self.grid[row as usize][column as usize];
            self.grid[row as usize][column as usize] = 0;
        }
    }
}

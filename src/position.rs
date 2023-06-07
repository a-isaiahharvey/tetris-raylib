use std::os::raw::c_int;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: c_int,
    pub column: c_int,
}

impl Position {
    pub fn new(row: c_int, column: c_int) -> Self {
        Self { row, column }
    }
}

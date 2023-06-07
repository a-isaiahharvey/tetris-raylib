use std::{collections::HashMap, os::raw::c_int};

use raylib::ffi::{Color, DrawRectangle};

use crate::{colors::get_cell_colors, position::Position};

#[derive(Debug, Default, Clone)]
pub struct Block {
    pub id: c_int,
    pub cells: HashMap<c_int, Vec<Position>>,
    cell_size: c_int,
    rotation_state: c_int,
    colors: Vec<Color>,
    row_offset: c_int,
    column_offset: c_int,
}

impl Block {
    pub fn new() -> Self {
        Self {
            cell_size: 30,
            rotation_state: 0,
            colors: get_cell_colors(),
            row_offset: 0,
            column_offset: 0,
            ..Default::default()
        }
    }

    pub fn draw(&mut self, offset_x: c_int, offset_y: c_int) {
        let tiles = self.get_cell_positions();
        for item in tiles {
            unsafe {
                DrawRectangle(
                    item.column * self.cell_size + offset_x,
                    item.row * self.cell_size + offset_y,
                    self.cell_size - 1,
                    self.cell_size - 1,
                    self.colors[self.id as usize],
                );
            }
        }
    }

    pub fn r#move(&mut self, rows: c_int, columns: c_int) {
        self.row_offset += rows;
        self.column_offset += columns;
    }

    pub fn get_cell_positions(&self) -> Vec<Position> {
        let tiles = &self.cells[&self.rotation_state];
        let mut moved_tiles = Vec::new();

        for item in tiles {
            let new_pos =
                Position::new(item.row + self.row_offset, item.column + self.column_offset);
            moved_tiles.push(new_pos);
        }

        moved_tiles
    }

    pub fn rotate(&mut self) {
        self.rotation_state += 1;

        if self.rotation_state == self.cells.len() as c_int {
            self.rotation_state = 0;
        }
    }

    pub fn undo_rotation(&mut self) {
        self.rotation_state -= 1;

        if self.rotation_state == -1 {
            self.rotation_state = self.cells.len() as c_int - 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockKind {
    L(Block),
    J(Block),
    I(Block),
    O(Block),
    S(Block),
    T(Block),
    Z(Block),
}

impl BlockKind {
    pub fn get_all_blocks() -> Vec<BlockKind> {
        vec![
            Self::I(Self::iblock()),
            Self::J(Self::jblock()),
            Self::L(Self::lblock()),
            Self::O(Self::oblock()),
            Self::S(Self::sblock()),
            Self::T(Self::tblock()),
            Self::Z(Self::zblock()),
        ]
    }

    pub fn lblock() -> Block {
        let mut block = Block {
            id: 1,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 2),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(2, 1),
                        Position::new(2, 2),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(0, 2),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 0),
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(2, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 3);
        block
    }

    pub fn jblock() -> Block {
        let mut block = Block {
            id: 2,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 0),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 1),
                        Position::new(0, 2),
                        Position::new(1, 1),
                        Position::new(2, 1),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 2),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(2, 0),
                        Position::new(2, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 3);
        block
    }

    pub fn iblock() -> Block {
        let mut block = Block {
            id: 3,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(1, 3),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 2),
                        Position::new(1, 2),
                        Position::new(2, 2),
                        Position::new(3, 2),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(2, 0),
                        Position::new(2, 1),
                        Position::new(2, 2),
                        Position::new(2, 3),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(2, 1),
                        Position::new(3, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(-1, 3);
        block
    }

    pub fn oblock() -> Block {
        let mut block = Block {
            id: 4,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 0),
                        Position::new(0, 1),
                        Position::new(1, 0),
                        Position::new(1, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 4);
        block
    }

    pub fn sblock() -> Block {
        let mut block = Block {
            id: 5,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 1),
                        Position::new(0, 2),
                        Position::new(1, 0),
                        Position::new(1, 1),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 2),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 0),
                        Position::new(2, 1),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 0),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(2, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 3);
        block
    }

    pub fn tblock() -> Block {
        let mut block = Block {
            id: 6,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 1),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 1),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(2, 1),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 3);
        block
    }

    pub fn zblock() -> Block {
        let mut block = Block {
            id: 7,
            cells: {
                let mut map = HashMap::new();
                map.insert(
                    0,
                    vec![
                        Position::new(0, 0),
                        Position::new(0, 1),
                        Position::new(1, 1),
                        Position::new(1, 2),
                    ],
                );

                map.insert(
                    1,
                    vec![
                        Position::new(0, 2),
                        Position::new(1, 1),
                        Position::new(1, 2),
                        Position::new(2, 1),
                    ],
                );

                map.insert(
                    2,
                    vec![
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(2, 1),
                        Position::new(2, 2),
                    ],
                );

                map.insert(
                    3,
                    vec![
                        Position::new(0, 1),
                        Position::new(1, 0),
                        Position::new(1, 1),
                        Position::new(2, 0),
                    ],
                );

                map
            },
            ..Block::new()
        };

        block.r#move(0, 3);
        block
    }
}

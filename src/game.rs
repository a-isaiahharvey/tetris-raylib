use std::{ffi::CString, os::raw::c_int};

use raylib::{
    ffi::{
        rand, CloseAudioDevice, GetKeyPressed, InitAudioDevice, LoadMusicStream, LoadSound, Music,
        PlayMusicStream, PlaySound, Sound, UnloadMusicStream, UnloadSound,
    },
    prelude::KeyboardKey,
};

use crate::{
    block::{Block, BlockKind},
    grid::Grid,
};

#[derive(Debug)]
pub struct Game {
    pub game_over: bool,
    pub score: c_int,
    pub music: Music,
    grid: Grid,
    blocks: Vec<BlockKind>,
    current_block: Option<Block>,
    next_block: Option<Block>,
    rotate_sound: Sound,
    clear_sound: Sound,
}

impl Game {
    pub fn new() -> Self {
        let music_file_path = CString::new("assets/sounds/music.mp3").unwrap();
        let rotate_file_path = CString::new("assets/sounds/rotate.mp3").unwrap();
        let clear_file_path = CString::new("assets/sounds/clear.mp3").unwrap();

        unsafe {
            InitAudioDevice();
        }

        let mut game = Self {
            game_over: false,
            score: 0,
            music: unsafe { LoadMusicStream(music_file_path.as_ptr()) },
            grid: Grid::new(),
            blocks: BlockKind::get_all_blocks(),
            current_block: None,
            next_block: None,
            rotate_sound: unsafe { LoadSound(rotate_file_path.as_ptr()) },
            clear_sound: unsafe { LoadSound(clear_file_path.as_ptr()) },
        };

        unsafe {
            PlayMusicStream(game.music);
        }

        game.current_block = Some(game.get_random_block());
        game.next_block = Some(game.get_random_block());

        game
    }

    pub fn draw(&mut self) {
        self.grid.draw();
        self.current_block.as_mut().unwrap().draw(11, 11);

        if let Some(next_block) = &mut self.next_block {
            match next_block.id {
                3 => next_block.draw(255, 290),
                4 => next_block.draw(255, 280),
                _ => next_block.draw(270, 270),
            };
        }
    }

    pub fn handle_input(&mut self) {
        let key_pressed = unsafe { GetKeyPressed() };

        if self.game_over && key_pressed != 0 {
            self.game_over = false;
            self.reset();
        }

        match key_pressed {
            key if key == KeyboardKey::KEY_LEFT as i32 => self.move_block_left(),
            key if key == KeyboardKey::KEY_RIGHT as i32 => self.move_block_right(),
            key if key == KeyboardKey::KEY_DOWN as i32 => {
                self.move_block_down();
                self.update_score(0, 1);
            }
            key if key == KeyboardKey::KEY_UP as i32 => self.rotate_block(),
            _ => (),
        }
    }

    pub fn move_block_down(&mut self) {
        if !self.game_over {
            self.current_block.as_mut().unwrap().r#move(1, 0);
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.as_mut().unwrap().r#move(-1, 0);
                self.lock_block();
            }
        }
    }

    fn move_block_left(&mut self) {
        if !self.game_over {
            self.current_block.as_mut().unwrap().r#move(0, -1);
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.as_mut().unwrap().r#move(0, 1);
            }
        }
    }

    fn move_block_right(&mut self) {
        if !self.game_over {
            self.current_block.as_mut().unwrap().r#move(0, 1);
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.as_mut().unwrap().r#move(0, -1);
            }
        }
    }

    fn get_random_block(&mut self) -> Block {
        if self.blocks.is_empty() {
            self.blocks = BlockKind::get_all_blocks();
        }

        let random_index = unsafe { rand() as usize % self.blocks.len() };
        let block = self.blocks[random_index].clone();
        self.blocks.remove(random_index);

        match block {
            BlockKind::L(block)
            | BlockKind::J(block)
            | BlockKind::I(block)
            | BlockKind::O(block)
            | BlockKind::S(block)
            | BlockKind::T(block)
            | BlockKind::Z(block) => block,
        }
    }

    fn is_block_outside(&self) -> bool {
        let tiles = self.current_block.as_ref().unwrap().get_cell_positions();

        for item in tiles {
            if self.grid.is_cell_outside(item.row, item.column) {
                return true;
            }
        }

        false
    }

    fn rotate_block(&mut self) {
        if !self.game_over {
            self.current_block.as_mut().unwrap().rotate();
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.as_mut().unwrap().undo_rotation();
            } else {
                unsafe {
                    PlaySound(self.rotate_sound);
                }
            }
        }
    }

    fn lock_block(&mut self) {
        let tiles = self.current_block.as_ref().unwrap().get_cell_positions();
        for item in tiles {
            self.grid.grid[item.row as usize][item.column as usize] =
                self.current_block.as_ref().unwrap().id;
        }
        self.current_block = self.next_block.clone();
        if !self.block_fits() {
            self.game_over = true;
        }
        self.next_block = Some(self.get_random_block());
        let rows_cleared = self.grid.clear_full_rows();
        if rows_cleared > 0 {
            unsafe {
                PlaySound(self.clear_sound);
            }
            self.update_score(rows_cleared, 0);
        }
    }

    fn block_fits(&mut self) -> bool {
        let tiles = self.current_block.as_ref().unwrap().get_cell_positions();
        for item in tiles {
            if !self.grid.is_cell_empty(item.row, item.column) {
                return false;
            }
        }

        true
    }

    fn reset(&mut self) {
        self.grid.initialize();
        self.blocks = BlockKind::get_all_blocks();
        self.current_block = Some(self.get_random_block());
        self.next_block = Some(self.get_random_block());
        self.score = 0;
    }

    fn update_score(&mut self, lines_cleared: c_int, move_down_points: c_int) {
        match lines_cleared {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            _ => (),
        }

        self.score += move_down_points;
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        unsafe {
            UnloadSound(self.rotate_sound);
            UnloadSound(self.clear_sound);
            UnloadMusicStream(self.music);
            CloseAudioDevice();
        }
    }
}

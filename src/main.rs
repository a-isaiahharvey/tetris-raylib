use std::ffi::CString;

use colors::{DARK_BLUE, LIGHT_BLUE};
use game::Game;
use raylib::{
    ffi::{
        BeginDrawing, ClearBackground, CloseWindow, DrawRectangleRounded, DrawTextEx, EndDrawing,
        GetTime, InitWindow, LoadFontEx, MeasureTextEx, Rectangle, SetTargetFPS, UpdateMusicStream,
        Vector2, WindowShouldClose,
    },
    prelude::Color,
};

static mut LAST_UPDATE_TIME: f64 = 0.0;

mod block;
mod colors;
mod game;
mod grid;
mod position;

fn event_triggered(interval: f64) -> bool {
    let current_time = unsafe { GetTime() };
    if unsafe { current_time - LAST_UPDATE_TIME >= interval } {
        unsafe {
            LAST_UPDATE_TIME = current_time;
        }

        return true;
    }

    false
}

fn main() {
    unsafe {
        let window_title = CString::new("Tetris").unwrap();
        InitWindow(500, 620, window_title.as_ptr());
        SetTargetFPS(60);

        let font_file_name = CString::new("assets/font/monogram.ttf").unwrap();

        let font = LoadFontEx(font_file_name.as_ptr(), 64, std::ptr::null_mut(), 0);

        let mut game = Game::new();

        while !WindowShouldClose() {
            UpdateMusicStream(game.music);
            game.handle_input();
            if event_triggered(0.2) {
                game.move_block_down();
            }

            BeginDrawing();
            ClearBackground(DARK_BLUE);

            let score_text = CString::new("Score").unwrap();
            DrawTextEx(
                font,
                score_text.as_ptr(),
                Vector2 { x: 365.0, y: 15.0 },
                38.0,
                2.0,
                Color::WHITE.into(),
            );

            let next_text = CString::new("Next").unwrap();
            DrawTextEx(
                font,
                next_text.as_ptr(),
                Vector2 { x: 370.0, y: 175.0 },
                38.0,
                2.0,
                Color::WHITE.into(),
            );

            if game.game_over {
                let game_over_text = CString::new("GAME OVER").unwrap();
                DrawTextEx(
                    font,
                    game_over_text.as_ptr(),
                    Vector2 { x: 320.0, y: 450.0 },
                    38.0,
                    2.0,
                    Color::WHITE.into(),
                );
            }

            DrawRectangleRounded(
                Rectangle {
                    x: 320.0,
                    y: 55.0,
                    width: 170.0,
                    height: 60.0,
                },
                0.3,
                6,
                LIGHT_BLUE,
            );

            let score_text = {
                let text = format!("{}", game.score);

                CString::new(text).unwrap()
            };

            let text_size = MeasureTextEx(font, score_text.as_ptr(), 38.0, 2.0);

            DrawTextEx(
                font,
                score_text.as_ptr(),
                Vector2 {
                    x: 320.0 + (170.0 - text_size.x) / 2.0,
                    y: 65.0,
                },
                38.0,
                2.0,
                Color::WHITE.into(),
            );
            DrawRectangleRounded(
                Rectangle {
                    x: 320.0,
                    y: 215.0,
                    width: 170.0,
                    height: 180.0,
                },
                0.3,
                6,
                LIGHT_BLUE,
            );
            game.draw();
            EndDrawing();
        }

        CloseWindow();
    }
}

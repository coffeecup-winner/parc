use std::cmp::{max, min};
use std::path::Path;

use pancurses::{cbreak, curs_set, endwin, initscr, noecho, Input, A_BOLD};

use crate::fs::directory::Directory;

mod fs;

fn main() {
    let window = initscr();
    window.refresh();
    window.keypad(true);
    cbreak();
    curs_set(0);
    noecho();
    let mut directory = Directory::open(&Path::new(".").canonicalize().unwrap());
    let mut selected_line_idx = 0;
    let mut status_line = String::new();
    loop {
        window.erase();
        window.mvprintw(0, 0, directory.path().to_str().unwrap());
        let mut y = 1;
        for filename in directory.entries() {
            window.mvprintw(y, 0, &filename.name);
            y += 1;
        }
        window.mvchgat(selected_line_idx + 1, 0, -1, A_BOLD, 0);
        window.mvprintw(window.get_max_y() - 1, 0, &status_line);
        window.refresh();
        match window.getch() {
            Some(Input::KeyF10) | Some(Input::Character('q')) => break,
            Some(Input::KeyDown) => {
                selected_line_idx = min(directory.entries().len() as i32 - 1, selected_line_idx + 1);
            }
            Some(Input::KeyUp) => {
                selected_line_idx = max(0, selected_line_idx - 1);
            }
            Some(Input::KeyEnter) | Some(Input::Character('\n')) => {
                if directory.open_subdirectory(selected_line_idx as usize) {
                    selected_line_idx = 0;
                }
            }
            Some(Input::KeyBackspace) | Some(Input::Character('\u{7f}')) => {
                if directory.open_parent() {
                    selected_line_idx = 0;
                }
            }
            Some(input) => {
                status_line = format!("Unhandled input: {:?}", input);
            }
            None => {}
        }
    }
    endwin();
}

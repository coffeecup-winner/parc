use std::cmp::{max, min};
use std::fs;
use std::path::Path;

use pancurses::{cbreak, curs_set, endwin, initscr, noecho, Input, A_BOLD};

struct FsEntry {
    pub name: String,
    pub type_: fs::FileType,
    pub metadata: fs::Metadata,
}

fn list_directory(path: &Path) -> Vec<FsEntry> {
    let mut result = vec![];
    match fs::read_dir(path) {
        Err(_) => {}
        Ok(d) => {
            for x in d {
                match x {
                    Err(_) => {}
                    Ok(de) => {
                        result.push(FsEntry {
                            name: de.file_name().into_string().unwrap(),
                            type_: de.file_type().unwrap(),
                            metadata: de.metadata().unwrap(),
                        });
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let window = initscr();
    window.refresh();
    window.keypad(true);
    cbreak();
    curs_set(0);
    noecho();
    let mut selected_line_idx = 0;
    let mut current_path = Path::new(".").canonicalize().unwrap();
    let mut status_line = String::new();
    loop {
        window.erase();
        let contents = list_directory(&current_path);
        window.mvprintw(0, 0, current_path.to_str().unwrap());
        let mut y = 1;
        for filename in &contents {
            window.mvprintw(y, 0, &filename.name);
            y += 1;
        }
        window.mvchgat(selected_line_idx + 1, 0, -1, A_BOLD, 0);
        window.mvprintw(window.get_max_y() - 1, 0, &status_line);
        window.refresh();
        match window.getch() {
            Some(Input::KeyF10) | Some(Input::Character('q')) => break,
            Some(Input::KeyDown) => {
                selected_line_idx = min(contents.len() as i32 - 1, selected_line_idx + 1);
            }
            Some(Input::KeyUp) => {
                selected_line_idx = max(0, selected_line_idx - 1);
            }
            Some(Input::KeyEnter) | Some(Input::Character('\n')) => {
                if contents[selected_line_idx as usize].type_.is_dir() {
                    current_path.push(&contents[selected_line_idx as usize].name);
                    selected_line_idx = 0;
                }
            }
            Some(Input::KeyBackspace) | Some(Input::Character('\u{7f}')) => {
                current_path = current_path.parent().unwrap_or(&current_path).to_path_buf();
                selected_line_idx = 0;
            }
            Some(input) => {
                status_line = format!("Unhandled input: {:?}", input);
            }
            None => {}
        }
    }
    endwin();
}

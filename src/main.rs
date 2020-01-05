use std::path::Path;

use pancurses::Input;

use crate::fs::directory::Directory;
use crate::ui::UI;

mod fs;
mod ui;

fn main() {
    let mut ui = UI::new();
    let mut directory = Directory::open(&Path::new(".").canonicalize().unwrap());
    loop {
        ui.clear();
        ui.draw(&directory);
        ui.refresh();
        match ui.get_next_input() {
            Input::KeyF10 | Input::Character('q') => break,
            Input::KeyDown => {
                if ui.selected_line_idx < directory.entries().len() as u32 - 1 {
                    ui.selected_line_idx += 1;
                }
            }
            Input::KeyUp => {
                if ui.selected_line_idx > 0 {
                    ui.selected_line_idx -= 1;
                }
            }
            Input::KeyEnter | Input::Character('\n') => {
                if directory.open_subdirectory(ui.selected_line_idx) {
                    ui.selected_line_idx = 0;
                }
            }
            Input::KeyBackspace | Input::Character('\u{7f}') => {
                if directory.open_parent() {
                    ui.selected_line_idx = 0;
                }
            }
            input => {
                ui.set_status(&format!("Unhandled input: {:?}", input));
            }
        }
    }
}

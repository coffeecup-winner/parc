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
                directory.select_next();
            }
            Input::KeyUp => {
                directory.select_previous();
            }
            Input::KeyEnter | Input::Character('\n') => {
                directory.open_selected_subdirectory();
            }
            Input::KeyBackspace | Input::Character('\u{7f}') => {
                directory.open_parent();
            }
            input => {
                ui.set_status(&format!("Unhandled input: {:?}", input));
            }
        }
    }
}

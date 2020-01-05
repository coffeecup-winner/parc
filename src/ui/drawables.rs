use pancurses::*;

use crate::fs::directory::Directory;
use crate::ui::{color, Drawable};

impl Drawable for Directory {
    fn draw(&self, window: &Window, selected_line_idx: u32) {
        window.mvprintw(0, 0, self.path().to_str().unwrap());
        let mut y = 1;
        for entry in self.entries() {
            let mut attr = 0;
            if entry.type_.is_dir() {
                attr = color::fg(COLOR_BLUE);
            }
            if y - 1 == selected_line_idx {
                attr |= A_REVERSE;
            }
            window.attrset(attr);
            window.mvprintw(y as i32, 0, &entry.name);
            y += 1;
        }
    }
}

use std::cmp::min;

use pancurses::*;

use crate::providers::Directory;
use crate::ui::{color, Drawable, View};

impl Drawable for Directory {
    fn draw(&self, window: &Window, view: &View) {
        window.mvprintw(0, 0, self.path().to_str().unwrap());
        if self.entries().len() == 0 {
            return;
        }
        let mut y = 1;
        let from = min(self.entries().len() - 1, view.first_line_offset as usize);
        let to = min(
            self.entries().len(),
            (view.first_line_offset + view.height) as usize,
        );
        for entry in &self.entries()[from..to] {
            let mut attr = 0;
            if entry.type_.is_dir() {
                attr |= color::fg(COLOR_BLUE);
            }
            if view.first_line_offset + y - 1 == self.selected_entry_idx() as u32 {
                attr |= A_REVERSE;
            }
            window.attrset(attr);
            window.mvprintw(y as i32, 0, &entry.name);
            y += 1;
        }
    }
}

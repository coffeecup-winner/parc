use pancurses::*;

use crate::fs::directory::Directory;
use crate::ui::Drawable;

impl Drawable for Directory {
    fn draw(&self, window: &Window, selected_line_idx: u32) {
        window.mvprintw(0, 0, self.path().to_str().unwrap());
        let mut y = 1;
        for filename in self.entries() {
            window.mvprintw(y, 0, &filename.name);
            y += 1;
        }
        window.mvchgat(selected_line_idx as i32 + 1, 0, -1, A_BOLD, 0);
    }
}

use pancurses::{cbreak, curs_set, endwin, initscr, noecho, Input, Window, A_BOLD};

// TODO: use traits instead
use crate::fs::directory::Directory;

pub struct UI {
    window: Window,
    status_line: String,
    pub selected_line_idx: u32,
}

impl Drop for UI {
    fn drop(&mut self) {
        endwin();
    }
}

impl UI {
    pub fn new() -> UI {
        let window = initscr();
        window.refresh();
        window.keypad(true);
        cbreak();
        curs_set(0);
        noecho();
        UI {
            window: window,
            status_line: String::new(),
            selected_line_idx: 0,
        }
    }

    pub fn get_next_input(&self) -> Input {
        self.window.getch().unwrap()
    }

    pub fn clear(&self) {
        self.window.erase();
    }

    pub fn refresh(&self) {
        self.window
            .mvprintw(self.window.get_max_y() - 1, 0, &self.status_line);
        self.window.refresh();
    }

    pub fn draw(&self, directory: &Directory) {
        self.window.erase();
        self.window
            .mvprintw(0, 0, directory.path().to_str().unwrap());
        let mut y = 1;
        for filename in directory.entries() {
            self.window.mvprintw(y, 0, &filename.name);
            y += 1;
        }
        self.window
            .mvchgat(self.selected_line_idx as i32 + 1, 0, -1, A_BOLD, 0);
    }

    pub fn set_status(&mut self, status_line: &str) {
        self.status_line = String::from(status_line);
    }
}

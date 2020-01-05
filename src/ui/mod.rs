use pancurses::*;

mod color;
mod drawables;

pub trait Drawable {
    fn draw(&self, window: &Window, selected_line_idx: u32);
}

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
        color::init();
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
        self.window.attrset(A_NORMAL);
        self.window
            .mvprintw(self.window.get_max_y() - 1, 0, &self.status_line);
        self.window.refresh();
    }

    pub fn draw<T: Drawable>(&self, drawable: &T) {
        self.window.attrset(A_NORMAL);
        drawable.draw(&self.window, self.selected_line_idx);
    }

    pub fn set_status(&mut self, status_line: &str) {
        self.status_line = String::from(status_line);
    }
}

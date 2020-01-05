use pancurses::*;

mod color;
mod drawables;

pub trait Drawable {
    fn draw(&self, window: &Window, view: &View);
}

pub struct View {
    pub first_line_offset: u32,
    pub width: u32,
    pub height: u32,
}

pub struct UI {
    window: Window,
    status_line: String,
    view: View,
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
        let max_x = window.get_max_x() as u32;
        let max_y = window.get_max_y() as u32 - 1; // leaving the status line
        UI {
            window: window,
            status_line: String::new(),
            view: View {
                first_line_offset: 0,
                width: max_x,
                height: max_y,
            },
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
        drawable.draw(&self.window, &self.view);
    }

    pub fn set_status(&mut self, status_line: &str) {
        self.status_line = String::from(status_line);
    }
}

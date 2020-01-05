use pancurses::*;

use crate::providers::Provider;

mod color;
mod drawables;

pub use pancurses::Input;

pub trait Drawable {
    fn draw(&self, window: &Window, view: &View);
}

pub struct View {
    pub first_line_offset: u32,
    pub width: u32,
    pub height: u32,
}

pub struct UI {
    provider: Box<dyn Provider>,
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
    pub fn new(initial_provider: Box<dyn Provider>) -> UI {
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
            provider: initial_provider,
            window,
            status_line: String::new(),
            view: View {
                first_line_offset: 0,
                width: max_x,
                height: max_y,
            },
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            self.clear();
            self.draw(&self.provider);
            self.refresh();
            match self.get_next_input() {
                Input::KeyF10 | Input::Character('q') => break,
                Input::KeyPPage => {
                    if self.provider.lines_count() == 0 {
                        continue;
                    }
                    if self.view.first_line_offset >= self.view.height {
                        self.view.first_line_offset -= self.view.height;
                    } else {
                        self.view.first_line_offset = 0;
                    }
                    self.provider.handle_window_scrolled(&self.view);
                }
                Input::KeyNPage => {
                    if self.provider.lines_count() == 0 {
                        continue;
                    }
                    if self.view.first_line_offset + self.view.height < self.provider.lines_count()
                    {
                        self.view.first_line_offset += self.view.height;
                    } else {
                        self.view.first_line_offset = self.provider.lines_count() - 1;
                    }
                    self.provider.handle_window_scrolled(&self.view);
                }
                input if self.provider.handle_input(&input) => {}
                input => {
                    self.set_status(&format!("Unhandled input: {:?}", input));
                }
            }
        }
    }

    fn get_next_input(&self) -> Input {
        self.window.getch().unwrap()
    }

    fn clear(&self) {
        self.window.erase();
    }

    fn refresh(&self) {
        self.window.attrset(A_NORMAL);
        self.window
            .mvprintw(self.window.get_max_y() - 1, 0, &self.status_line);
        self.window.refresh();
    }

    fn draw(&self, drawable: &Box<dyn Provider>) {
        self.window.attrset(A_NORMAL);
        drawable.draw(&self.window, &self.view);
    }

    pub fn set_status(&mut self, status_line: &str) {
        self.status_line = String::from(status_line);
    }
}

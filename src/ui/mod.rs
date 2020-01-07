use pancurses::*;

use crate::providers::Provider;

mod color;
mod drawables;

pub use pancurses::Input;

pub trait Drawable {
    fn draw(&self, window: &Window, view: &View);
}

#[derive(Clone)]
pub struct View {
    first_line_offset: u32,
    width: u32,
    height: u32,
}

impl View {
    pub fn first_line_offset(&self) -> u32 {
        self.first_line_offset
    }

    pub fn set_first_line_offset(&mut self, value: u32) {
        self.first_line_offset = value;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct UI {
    main_provider: Box<dyn Provider>,
    main_view: View,
    background_provider: Box<dyn Provider>,
    background_view: View,
    window: Window,
    status_line: String,
}

impl Drop for UI {
    fn drop(&mut self) {
        endwin();
    }
}

impl UI {
    pub fn new(main_provider: Box<dyn Provider>, background_provider: Box<dyn Provider>) -> UI {
        let window = initscr();
        window.refresh();
        window.keypad(true);
        cbreak();
        curs_set(0);
        noecho();
        color::init();
        let max_x = window.get_max_x() as u32;
        let max_y = window.get_max_y() as u32 - 1; // leaving the status line
        let view = View {
            first_line_offset: 0,
            width: max_x,
            height: max_y,
        };
        UI {
            main_provider,
            main_view: view.clone(),
            background_provider,
            background_view: view,
            window,
            status_line: String::new(),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            self.clear();
            self.window.attrset(A_NORMAL);
            self.main_provider.draw(&self.window, &self.main_view);
            self.refresh();
            match self.get_next_input() {
                Input::KeyF10 | Input::Character('q') => break,
                Input::Character('\t') => {
                    std::mem::swap(&mut self.main_provider, &mut self.background_provider);
                    std::mem::swap(&mut self.main_view, &mut self.background_view);
                }
                Input::KeyPPage => {
                    if self.main_provider.lines_count() == 0 {
                        continue;
                    }
                    if self.main_view.first_line_offset >= self.main_view.height {
                        self.main_view.first_line_offset -= self.main_view.height;
                    } else {
                        self.main_view.first_line_offset = 0;
                    }
                    self.main_provider.handle_window_scrolled(&self.main_view);
                }
                Input::KeyNPage => {
                    if self.main_provider.lines_count() == 0 {
                        continue;
                    }
                    if self.main_view.first_line_offset + self.main_view.height
                        < self.main_provider.lines_count()
                    {
                        self.main_view.first_line_offset += self.main_view.height;
                    } else {
                        self.main_view.first_line_offset = self.main_provider.lines_count() - 1;
                    }
                    self.main_provider.handle_window_scrolled(&self.main_view);
                }
                input if self.main_provider.handle_input(&input, &mut self.main_view) => {}
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

    pub fn set_status(&mut self, status_line: &str) {
        self.status_line = String::from(status_line);
    }
}

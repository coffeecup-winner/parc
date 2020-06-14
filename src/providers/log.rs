use std::cmp::min;
use std::path::Path;
use std::sync::Mutex;

use log::{Level, Metadata, Record};
use pancurses::Window;

use crate::providers::directory::FsEntry; // TODO: move FsEntry from there
use crate::providers::Provider;
use crate::ui::{Drawable, Input, View};

lazy_static! {
    static ref LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub struct Logger {}

impl Logger {
    pub fn new() -> Box<Logger> {
        Box::new(Logger {})
    }
}

impl ::log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            LINES.lock().unwrap().push(format!("{}", record.args()));
        }
    }

    fn flush(&self) {}
}

pub struct Log {}

impl Log {
    pub fn new() -> Box<Log> {
        Box::new(Log {})
    }
}

impl Drawable for Log {
    fn draw(&self, window: &Window, view: &View) {
        let count = self.lines_count();
        if count == 0 {
            return;
        }
        let mut y = 0;
        let from = min(count - 1, view.first_line_offset());
        let to = min(count, view.first_line_offset() + view.height());
        for entry in &LINES.lock().unwrap()[from as usize..to as usize] {
            window.mvprintw(y as i32, 0, entry);
            y += 1;
        }
    }
}

impl Provider for Log {
    fn lines_count(&self) -> u32 {
        LINES.lock().unwrap().len() as u32
    }

    fn handle_input(
        &mut self,
        _input: &Input,
        _view: &mut View,
        _other: &mut Box<dyn Provider>,
    ) -> bool {
        false
    }

    fn handle_window_scrolled(&mut self, _view: &View) {}

    fn copy_to(&mut self, _path: &Path, _entries: &Vec<FsEntry>) -> bool {
        false
    }
}

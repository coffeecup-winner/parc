use std::path::Path;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::providers::{Directory, Log, Logger};
use crate::ui::{Input, UI};

mod providers;
mod ui;

fn main() {
    log::set_boxed_logger(Logger::new()).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    info!("parc started");
    let current_path = Path::new(".").canonicalize().unwrap();
    let main_provider = Box::new(Directory::open(&current_path));
    let background_provider = Box::new(Directory::open(&current_path));
    let mut ui = UI::new(main_provider, background_provider);
    ui.add_provider(Input::Character('l'), Log::new());
    ui.main_loop();
}

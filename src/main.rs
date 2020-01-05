use std::path::Path;

use crate::providers::Directory;
use crate::ui::UI;

mod providers;
mod ui;

fn main() {
    let directory = Box::new(Directory::open(&Path::new(".").canonicalize().unwrap()));
    let mut ui = UI::new(directory);
    ui.main_loop();
}

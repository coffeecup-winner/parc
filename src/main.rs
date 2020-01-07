use std::path::Path;

use crate::providers::Directory;
use crate::ui::UI;

mod providers;
mod ui;

fn main() {
    let main_provider = Box::new(Directory::open(&Path::new(".").canonicalize().unwrap()));
    let background_provider = Box::new(Directory::open(&Path::new(".").canonicalize().unwrap()));
    let mut ui = UI::new(main_provider, background_provider);
    ui.main_loop();
}

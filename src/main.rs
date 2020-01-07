use std::path::Path;

use crate::providers::Directory;
use crate::ui::UI;

mod providers;
mod ui;

fn main() {
    let current_path = Path::new(".").canonicalize().unwrap();
    let main_provider = Box::new(Directory::open(&current_path));
    let background_provider = Box::new(Directory::open(&current_path));
    let mut ui = UI::new(main_provider, background_provider);
    ui.main_loop();
}

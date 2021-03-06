use crate::ui::{Drawable, Input, View};

pub mod directory;
pub mod log;

pub use self::log::{Log, Logger};
pub use directory::Directory;

pub trait Provider: Drawable {
    fn lines_count(&self) -> u32;
    fn handle_input(&mut self, input: &Input, view: &mut View) -> bool;
    fn handle_window_scrolled(&mut self, view: &View);
}

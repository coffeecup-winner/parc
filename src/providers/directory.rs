use std::cmp::{max, min, Ordering};
use std::fs;
use std::path::{Path, PathBuf};

use crate::providers::Provider;
use crate::ui::{Input, View};

pub struct FsEntry {
    pub name: String,
    pub type_: fs::FileType,
    pub metadata: fs::Metadata,
}

pub struct Directory {
    path: PathBuf,
    entries: Vec<FsEntry>,
    selected_entry_idx: usize,
}

impl Directory {
    pub fn open(path: &Path) -> Directory {
        let mut directory = Directory {
            path: path.to_path_buf(),
            entries: vec![],
            selected_entry_idx: 0,
        };
        directory.refresh();
        directory
    }

    fn refresh(&mut self) {
        self.entries.clear();
        if let Ok(d) = fs::read_dir(&self.path) {
            for x in d {
                if let Ok(de) = x {
                    self.entries.push(FsEntry {
                        name: de.file_name().into_string().unwrap(),
                        type_: de.file_type().unwrap(),
                        metadata: de.metadata().unwrap(),
                    });
                }
            }
        }
        self.entries.sort_by(|a, b| {
            if a.type_.is_dir() ^ b.type_.is_dir() {
                if a.type_.is_dir() {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                a.name.cmp(&b.name)
            }
        });
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn entries(&self) -> &Vec<FsEntry> {
        &self.entries
    }

    pub fn selected_entry_idx(&self) -> usize {
        self.selected_entry_idx
    }

    fn select_previous(&mut self) {
        if self.selected_entry_idx > 0 {
            self.selected_entry_idx -= 1;
        }
    }

    fn select_next(&mut self) {
        if self.selected_entry_idx < self.entries.len() as usize - 1 {
            self.selected_entry_idx += 1;
        }
    }

    fn open_selected_subdirectory(&mut self) {
        let entry = &self.entries[self.selected_entry_idx];
        if entry.type_.is_dir() {
            self.path.push(&entry.name);
            self.refresh();
            self.selected_entry_idx = 0;
        }
    }

    fn open_parent(&mut self) {
        if let Some(parent) = self.path.parent() {
            let current_directory_name =
                self.path.file_name().unwrap().to_str().unwrap().to_owned();
            self.path = parent.to_path_buf();
            self.refresh();
            self.selected_entry_idx = self
                .entries
                .iter()
                .position(|e| e.name == current_directory_name)
                .unwrap_or(0);
        }
    }
}

impl Provider for Directory {
    fn handle_input(&mut self, input: &Input) -> bool {
        match input {
            Input::KeyDown => {
                self.select_next();
            }
            Input::KeyUp => {
                self.select_previous();
            }
            Input::KeyEnter | Input::Character('\n') => {
                self.open_selected_subdirectory();
            }
            Input::KeyBackspace | Input::Character('\u{7f}') => {
                self.open_parent();
            }
            _ => return false,
        }
        true
    }

    fn handle_window_scrolled(&mut self, view: &View) {
        self.selected_entry_idx = max(
            view.first_line_offset + view.height,
            min(view.first_line_offset, self.selected_entry_idx as u32),
        ) as usize;
    }
}

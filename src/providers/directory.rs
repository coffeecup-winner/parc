use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn select_previous(&mut self) {
        if self.selected_entry_idx > 0 {
            self.selected_entry_idx -= 1;
        }
    }

    pub fn select_next(&mut self) {
        if self.selected_entry_idx < self.entries.len() as usize - 1 {
            self.selected_entry_idx += 1;
        }
    }

    pub fn open_selected_subdirectory(&mut self) {
        let entry = &self.entries[self.selected_entry_idx];
        if entry.type_.is_dir() {
            self.path.push(&entry.name);
            self.refresh();
            self.selected_entry_idx = 0;
        }
    }

    pub fn open_parent(&mut self) {
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

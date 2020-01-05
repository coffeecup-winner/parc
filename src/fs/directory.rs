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
}

impl Directory {
    pub fn open(path: &Path) -> Directory {
        let mut directory = Directory {
            path: path.to_path_buf(),
            entries: vec!(),
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
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn entries(&self) -> &Vec<FsEntry> {
        &self.entries
    }

    pub fn open_subdirectory(&mut self, idx: u32) -> bool {
        if idx as usize > self.entries.len() {
            return false;
        }
        let entry = &self.entries[idx as usize];
        if entry.type_.is_dir() {
            self.path.push(&entry.name);
            self.refresh();
            return true;
        }
        return false;
    }

    pub fn open_parent(&mut self) -> bool {
        if let Some(parent) = self.path.parent() {
            self.path = parent.to_path_buf();
            self.refresh();
            return true;
        }
        return false;
    }
}

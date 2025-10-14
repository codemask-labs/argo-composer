use std::path::PathBuf;

pub struct Template {
    pub path: Option<PathBuf>,
}

impl Template {
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn replace(&mut self) {}

    pub fn from_path(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    pub fn write(&self) {
        todo!()
    }

    pub fn write_to_path(&self, path: PathBuf) {
        todo!()
    }
}

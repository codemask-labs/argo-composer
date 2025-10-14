use std::path::PathBuf;

pub struct Resource {
    pub path: Option<PathBuf>,
}

impl Resource {
    pub fn from_path(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }
}

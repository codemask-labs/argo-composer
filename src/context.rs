use std::{env::current_dir, path::PathBuf};

pub struct Context {
    pub cwd: PathBuf,
}

impl Context {
    pub fn new() -> Self {
        let cwd = current_dir().unwrap();

        Self { cwd }
    }
}

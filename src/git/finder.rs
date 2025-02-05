use std::path::PathBuf;
use walkdir::WalkDir;

pub struct GitFinder {
    pub current_dir: PathBuf,
    original_dir: PathBuf,
}

impl GitFinder {
    pub fn new() -> std::io::Result<Self> {
        let current_dir = std::env::current_dir()?;
        Ok(Self {
            current_dir: current_dir.clone(),
            original_dir: current_dir,
        })
    }

    pub fn is_git_repo(&self, path: &PathBuf) -> bool {
        path.join(".git").is_dir()
    }

    pub fn get_subdirectories(&self) -> Vec<PathBuf> {
        WalkDir::new(&self.current_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .map(|e| e.path().to_path_buf())
            .collect()
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.current_dir = path;
    }

    pub fn navigate_to_parent(&mut self) -> bool {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            true
        } else {
            false
        }
    }

    pub fn navigate_to_root(&mut self) {
        self.current_dir = self.original_dir.clone();
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }
}

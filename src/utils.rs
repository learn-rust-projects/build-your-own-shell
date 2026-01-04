use std::path::{Path, PathBuf};

use is_executable::IsExecutable;
fn find_executable_file_in_path(path: &Path) -> Option<PathBuf> {
    if path.is_file() && path.is_executable() {
        return Some(path.to_path_buf());
    }
    None
}

pub fn find_executable_file_in_paths(
    executable_file: &str,
    paths: &Vec<PathBuf>,
) -> Option<PathBuf> {
    for path in paths {
        if (path.exists() || path.is_dir())
            && let Some(file_path) = find_executable_file_in_path(&path.join(executable_file))
        {
            return Some(file_path);
        }
    }
    None
}

use std::fs;

pub fn find_all_executable_file_in_paths(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| path.exists() && path.is_dir())
        .flat_map(|dir| {
            fs::read_dir(dir)
                .map(|rd| {
                    Box::new(
                        rd.filter_map(|entry| entry.ok())
                            .filter_map(|entry| find_executable_file_in_path(&entry.path())),
                    ) as Box<dyn Iterator<Item = PathBuf>>
                })
                .unwrap_or_else(|_| Box::new(std::iter::empty()))
        })
        .collect()
}

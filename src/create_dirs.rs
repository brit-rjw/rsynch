use walkdir::WalkDir;
use std::fs;
use crate::helpers::get_path_twin;

pub fn create_missing_dirs(src_path: &str, tgt_path: &str) {
    for file in WalkDir::new(src_path).into_iter() {
        match file {
            Ok(entry) => if entry.file_type().is_dir() {
                let t_path = get_path_twin(entry.path(), src_path, tgt_path);
                if !t_path.exists() {
                    match fs::create_dir(t_path.as_path()) { // Create missing directory
                        Ok(_) => {}
                        Err(err) => eprintln!("ERROR: {}", err)
                    }
                }
            }
            Err(err) => eprintln!("ERROR: {}", err)
        }
    }
}

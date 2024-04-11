use walkdir::WalkDir;
use std::fs;
use crate::helpers::get_path_twin;

pub fn remove_extra_dirs_and_files(tgt_path: &str, src_path: &str) {
    for file in WalkDir::new(tgt_path).into_iter() {
        match file {
            Ok(entry) => match entry.file_type().is_dir() {
                true => {
                    let s_path = get_path_twin(entry.path(), tgt_path, src_path);
                    if !s_path.exists() {
                        match fs::remove_dir_all(entry.path()) {
                            Ok(_) => {}
                            Err(err) => eprintln!("ERROR: {}", err)
                        }
                    }
                }
                false => {
                    let s_path = get_path_twin(entry.path(), tgt_path, src_path);
                    if !s_path.exists() {
                        match fs::remove_file(entry.path()) {
                            Ok(_) => {}
                            Err(err) => eprintln!("ERROR: {}", err)
                        }
                    }
                }
            }
            Err(err) => eprintln!("ERROR: {}", err)
        }
    }
}

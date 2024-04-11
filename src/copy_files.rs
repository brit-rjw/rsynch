use std::path::Path;
use std::fs::copy;
use walkdir::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::models::{FilePair,FileStatus};
use crate::helpers::{get_path_twin, same_file_size, same_file_content};

fn get_file_pairs(src_path: &str, tgt_path: &str) -> Vec<FilePair> {
    let mut pairs: Vec<FilePair> = Vec::new();
    for file in WalkDir::new(src_path).into_iter() {
        match file {
            Ok(entry) => if entry.file_type().is_file() {
                let t_path = get_path_twin(entry.path(), src_path, tgt_path);
                let status = if t_path.exists() {
                    match same_file_size(entry.path(), t_path.as_path()) {
                        Ok(true) => FileStatus::Compare,
                        Ok(false) => FileStatus::Copy,
                        Err(err) => {
                            eprintln!("ERROR: {}", err);
                            continue
                        }
                    }
                    } else { FileStatus::Copy };
                pairs.push(FilePair { status, src_path: entry.into_path(), tgt_path: t_path })
            }
            Err(err) => eprintln!("ERROR: {}", err)
        }
    }
    pairs
}

fn process_file_pairs(pairs: Vec<FilePair>) {
    pairs.par_iter().for_each(|fp| {
        let s_path = Path::new(&fp.src_path);
        let t_path = Path::new(&fp.tgt_path);
        match fp.status {
            FileStatus::Compare => {
                match same_file_content(&s_path.to_path_buf(), &t_path.to_path_buf()) {
                    Ok(true) => {}
                    Ok(false) => {
                        if let Err(err) = copy(s_path, t_path) { eprintln!("ERROR: {}", err) }
                    }
                    Err(err) => { eprintln!("ERROR: {}", err) }
                }
            }
            FileStatus::Copy => {
                if let Err(err) = copy(s_path, t_path) { eprintln!("ERROR: {}", err) }
            }
        }
    })
}

pub fn copy_files(src_path: &str, tgt_path: &str) {
    let file_pairs: Vec<FilePair> = get_file_pairs(src_path, tgt_path);
    process_file_pairs(file_pairs)
}
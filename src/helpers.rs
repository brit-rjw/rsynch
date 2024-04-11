use std::path::{Path, PathBuf};
use std::{fs, io};
use std::io::{BufReader, Read};
use std::fs::File;

pub fn get_path_twin(active_path: &Path, root_dir_path: &str, new_dir_root_path: &str) -> PathBuf {
    let path = active_path.strip_prefix(root_dir_path).unwrap();
    Path::new(new_dir_root_path).join(path)
}

pub fn same_file_size(src: &Path, tgt: &Path) -> io::Result<bool> {
    let s_file_meta = fs::metadata(src)?;
    let t_file_meta = fs::metadata(tgt)?;
    Ok(s_file_meta.len() == t_file_meta.len())
}

pub fn same_file_content(src_path: &PathBuf, tgt_path: &PathBuf) -> io::Result<bool> {
    const BUFF_SIZE: usize = 4096;
    let mut src_rdr = BufReader::new(File::open(src_path)?);
    let mut tgt_rdr = BufReader::new(File::open(tgt_path)?);
    let mut src_buff = [0; BUFF_SIZE];
    let mut tgt_buff = [0; BUFF_SIZE];

    loop {
        let src_bytes = src_rdr.read(&mut src_buff)?;
        let tgt_bytes = tgt_rdr.read(&mut tgt_buff)?;
        if src_bytes != tgt_bytes || src_buff[..src_bytes] != tgt_buff[..tgt_bytes] {
            return Ok(false)
        }
        if src_bytes == 0 && tgt_bytes == 0 { return Ok(true) }
    }
}

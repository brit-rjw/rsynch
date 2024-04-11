use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum FileStatus { Copy, Compare }

#[derive(Debug)]
pub struct FilePair { pub status: FileStatus, pub src_path: PathBuf, pub tgt_path: PathBuf }

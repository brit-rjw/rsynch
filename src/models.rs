use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args { pub source_dir_path: String, pub target_dir_path: String }

#[derive(Debug, PartialEq, Eq)]
pub enum FileStatus { Copy, Compare }

#[derive(Debug)]
pub struct FilePair { pub status: FileStatus, pub src_path: PathBuf, pub tgt_path: PathBuf }

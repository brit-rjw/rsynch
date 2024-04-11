use clap::Parser;
use crate::models::Args;

pub fn get_paths() -> Args {
    let args = Args::parse();
    Args { source_dir_path: args.source_dir_path, target_dir_path: args.target_dir_path }
}
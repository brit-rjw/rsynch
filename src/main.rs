mod models;
mod helpers;
mod remove_extras;
mod create_dirs;
mod copy_files;
mod get_args;

use crate::get_args::{get_paths};

fn main() {
    let args = get_paths();
    let src_path = args.source_dir_path.as_str();
    let tgt_path = args.target_dir_path.as_str();

    remove_extras::remove_extra_dirs_and_files(tgt_path, src_path);
    create_dirs::create_missing_dirs(src_path, tgt_path);
    copy_files::copy_files(src_path, tgt_path)
}

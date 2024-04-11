mod models;
mod helpers;
mod remove_extras;
mod create_dirs;
mod copy_files;

fn main() {
    let source_path: &str = "/Users/robertwoodward/Downloads/RSyncTestDirs";
    let target_path: &str = "/Users/robertwoodward/Downloads/RSyncTestDirs1";

    remove_extras::remove_extra_dirs_and_files(target_path, source_path);
    create_dirs::create_missing_dirs(source_path, target_path);
    copy_files::copy_files(source_path, target_path)
}

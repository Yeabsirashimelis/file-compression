pub mod args;
pub mod compression;
pub mod file_ops;

pub use args::Args;
pub use compression::compress_file;
pub use file_ops::{open_input_file, create_output_file, print_file_metadata};

use std::fs::File;
use std::io::BufReader;

pub fn open_input_file(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn create_output_file(path: &str) -> Result<File, std::io::Error> {
    File::create(path)
}

pub fn print_file_metadata(input: &BufReader<File>, output: &File) {
    let input_file_ref = input.get_ref();

    match input_file_ref.metadata() {
        Result::Ok(metadata) => {
            println!("Source len: {:?}", metadata.len());
        }
        Err(error) => {
            println!("Source length not found. Error: {}", error);
        }
    }

    match output.metadata() {
        Result::Ok(metadata) => {
            println!("Target len: {:?}", metadata.len());
        }
        Err(error) => {
            println!("output length not found. Error: {}", error);
        }
    }
}

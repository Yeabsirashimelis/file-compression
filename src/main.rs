use std::{
    env::args,
    fs::File,
    io::{BufReader, copy},
    time::Instant,
};

use flate2::{Compression, write::GzEncoder};

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let input_file = match args().nth(1) {
        Option::Some(path) => path,
        Option::None => {
            eprintln!("input file name dont't specified");
            return;
        }
    };

    let input_file_open = match File::open(input_file) {
        Result::Ok(file) => file,
        Result::Err(err) => {
            return eprintln!("error: {:?}", err);
        }
    };

    let mut input = BufReader::new(input_file_open);

    let output = match args().nth(2) {
        Option::Some(path) => path,
        Option::None => {
            eprintln!("output file name dont't specified");
            return;
        }
    };

    let output_file_path = if output.contains(".") {
        let output_file_path = output.split(".").collect::<Vec<&str>>()[0];
        format!("{}.gz", output_file_path)
    } else {
        format!("{}.gz", output)
    };

    println!("{output_file_path}");

    let output = match File::create(output_file_path) {
        Result::Ok(file) => file,
        Result::Err(error) => {
            return eprintln!("error: {:?}", error);
        }
    };
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    if let Result::Err(error) = copy(&mut input, &mut encoder) {
        eprintln!("error: {:?}", error)
    }

    let output = match encoder.finish() {
        Result::Ok(output_file) => output_file,
        Result::Err(error) => {
            return eprintln!("encoder finishes with error : {:?}", error);
        }
    };

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

    println!("Elapsed: {:?}", start.elapsed())
}

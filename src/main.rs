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

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = args().nth(2).unwrap();

    let output_file_path = if output.contains(".") {
        let output_file_path = output.split(".").collect::<Vec<&str>>()[0];
        format!("{}.zip", output_file_path)
    } else {
        format!("{}.zip", output)
    };

    println!("{output_file_path}");

    let output = File::create(output_file_path).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed())
}

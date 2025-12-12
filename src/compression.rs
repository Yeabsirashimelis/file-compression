use std::fs::File;
use std::io::{BufReader, copy};
use std::time::Instant;
use flate2::{Compression, write::GzEncoder};

pub fn compress_file(
    input: &mut BufReader<File>,
    output_path: &str,
) -> Result<File, std::io::Error> {
    let output = File::create(output_path)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    if let Result::Err(error) = copy(input, &mut encoder) {
        eprintln!("error: {:?}", error)
    }

    let output = encoder.finish()?;

    println!("Elapsed: {:?}", start.elapsed());

    Ok(output)
}

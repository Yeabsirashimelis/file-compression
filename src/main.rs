use file_compression::{Args, compress_file, open_input_file, print_file_metadata};

fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let mut input = match open_input_file(&args.input_file) {
        Ok(file) => file,
        Err(err) => {
            return eprintln!("error: {:?}", err);
        }
    };

    println!("{}", args.output_file);

    let output = match compress_file(&mut input, &args.output_file) {
        Ok(output_file) => output_file,
        Err(error) => {
            return eprintln!("encoder finishes with error : {:?}", error);
        }
    };

    print_file_metadata(&input, &output);
}

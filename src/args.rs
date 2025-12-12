use std::env::args;

pub struct Args {
    pub input_file: String,
    pub output_file: String,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        if args().len() != 3 {
            return Err("Usage: `source` `target`".to_string());
        }

        let input_file = match args().nth(1) {
            Option::Some(path) => path,
            Option::None => {
                return Err("input file name dont't specified".to_string());
            }
        };

        let output = match args().nth(2) {
            Option::Some(path) => path,
            Option::None => {
                return Err("output file name dont't specified".to_string());
            }
        };

        let output_file = if output.contains(".") {
            let output_file_path = output.split(".").collect::<Vec<&str>>()[0];
            format!("{}.gz", output_file_path)
        } else {
            format!("{}.gz", output)
        };

        Ok(Args {
            input_file,
            output_file,
        })
    }
}

use std::{env, fs, path::PathBuf, process::exit};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc < 2 {
        eprintln!("No input file provided");
        exit(1);
    } else if argc > 2 {
        eprintln!("Expected exactly one argument fournd {}", argc);
        exit(2);
    }

    let input_file = PathBuf::from(&argv[1]);

    let input_file_content = fs::read_to_string(&input_file);

    match input_file_content {
        Ok(_content) => todo!(),
        Err(err) => {
            eprintln!("{}", err);
            exit(3)
        }
    }
}

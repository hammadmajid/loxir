use std::{env, fs, path::PathBuf, process::exit};
use std::{
    env, fs,
    io::{stdin, Write},
    path::PathBuf,
    process::exit,
};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc > 2 {
        eprintln!("Usage:\n\trlox [script]");
        exit(64);
    } else if argc == 2 {
        let input_file = PathBuf::from(&argv[1]);

        let input_file_content = fs::read_to_string(&input_file);

        match input_file_content {
            Ok(_content) => todo!(),
            Err(err) => {
                eprintln!("{}", err);
                exit(66)
            }
        }
    } else {
        // REPL
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed

            let mut line = String::new();
            match stdin().read_line(&mut line) {
                Ok(_) => todo!(),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(65)
                }
            }
        }
    }
}


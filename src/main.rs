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
        run_file(input_file);
    } else {
        run_prompt();
    }
}

fn run_file(input_file: PathBuf) {
    let input_file_content = fs::read_to_string(&input_file);

    match input_file_content {
        Ok(content) => run(content, RunMode::File),
        Err(err) => {
            eprintln!("{}", err);
            exit(66)
        }
    }
}

fn run_prompt() -> ! {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => run(line, RunMode::REPL),
            Err(err) => {
                eprintln!("{}", err);
                exit(65)
            }
        }
    }
}

fn run(source: String, mode: RunMode) {

enum RunMode {
    // runs the code from a source file
    File,

    // runs the code inside an interactive REPL
    REPL,
}

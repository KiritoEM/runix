use std::io::{self, Write};

pub fn run_repl<F>(mut run: F)
where F: FnMut(&str)
 {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();

        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let line = input.trim();

        if line.is_empty() {
            break;
        }

        run(line);
    }
}

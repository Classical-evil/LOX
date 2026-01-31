use std::error::Error;

mod Scanner;
mod Token;
mod TokenType;

fn main() -> Result<(), Box<dyn Error>> {
    use std::env;
    use std::process;

    let args: Vec<String> = env::args().collect();
    match args.len() {
        len if len > 2 => {
            eprintln!("To many argsment!!");
            process::exit(64);
        }
        2 => {
            run_file(&args[1])?;
        }
        1 => {
            run_prompt()?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    use std::fs;

    let bytes = fs::read(path)?;
    let source = String::from_utf8(bytes)?;
    run(&source);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    use std::io::{self, Write};

    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let read_bytes = io::stdin().read_line(&mut line).unwrap();
        if read_bytes == 0 {
            break;
        }
        run(&line);
    }

    Ok(())
}

fn run(source: &str) {
    use crate::Scanner::Scanner;
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scanTokens();
    for token in tokens {
        print!("{} ", token);
    }
    println!();
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, postion: &str, message: &str) {
    println!("[line{}Error]{}:{}", line, postion, message);
}

extern crate getopts;

use getopts::Options;
use std::{env, fs};

fn emit(input: &str, output_path: Option<String>) {
    let emit_path = output_path.unwrap_or("public".to_string());
    println!("{}", emit_path);

    let data = fs::read_to_string(input).expect("failed to read input file");
    println!("{}", data);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output folder path", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    emit(&input, output);
}

extern crate getopts;

use getopts::Options;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Quote {
    text: String,
    author: String,
    auto_id: Option<bool>,
    custom_id: Option<String>,
    custom_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct QuoteData {
    path_prefix: Option<String>,
    quotes: Vec<Quote>,
}

fn emit(input: &str, output_path: Option<String>) {
    let emit_path = output_path.unwrap_or("public".to_string());
    println!("{}", emit_path);

    let data = fs::read_to_string(input).expect("failed to read input file");
    println!("{}", data);

    let quote_data: QuoteData = serde_yaml::from_str(&data).unwrap();

    println!("{:#?}", quote_data);

    // attempt creating emit folder
    std::fs::create_dir_all(PathBuf::from(&emit_path)).unwrap();

    let mut auto_id: usize = 1;

    for quote in quote_data.quotes {
        let text = format!("{{\"author\":\"{}\",\"text\":\"{}\"}}", quote.author, quote.text);

        let mut filepath = PathBuf::new();
        filepath.push(&emit_path);
        if quote.custom_path.is_some() {
            filepath.push(quote.custom_path.unwrap());
        } else {
            filepath.push(quote_data.path_prefix.clone().unwrap());
        }
        std::fs::create_dir_all(&filepath).unwrap();


        if quote.custom_id.is_some() {
            filepath.push(&quote.custom_id.unwrap());
            fs::write(&filepath, &text).unwrap();
            filepath.pop();
        }

        if quote.auto_id.is_none() || quote.auto_id.unwrap() == true {
            filepath.push(auto_id.to_string());
            fs::write(&filepath, &text).unwrap();
            auto_id = auto_id + 1;
        }
    }
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

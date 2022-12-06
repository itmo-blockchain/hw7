use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;
use sha3::{Digest, Sha3_256};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[command(
    version,
    author = "Ruzavin M.",
    about = "A application to generate task number for students",
)]
struct Args {
    /// Count of bilets
    #[arg(short = 'n', long = "numbilets")]
    numbilets: u64,

    /// Path to file with students
    #[arg(short = 'f', long = "file")]
    file: String,

    /// Parameter for randomize
    #[arg(short = 'p', long = "parameter")]
    parameter: u64,
}

fn get_bilet_num(s: &str, parameter: u64, max: u64) -> u64 {
    let mut hasher = Sha3_256::new();
    hasher.update(s);
    hasher.update(APP_NAME);
    hasher.update(parameter.to_le_bytes());
    let hash = hasher.finalize();

    let result = hash.iter()
        .step_by(4)
        .take(8)
        .map(|&x| x as u64)
        .reduce(|acc, x| (acc << 8) + x)
        .unwrap();
    result % max
}

fn main() {
    let args = Args::parse();

    let lines = File::open(&args.file)
        .map(BufReader::new)
        .map(BufRead::lines);
    
    let lines = match lines {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };
    
        println!("{}: {}", line, get_bilet_num(&line, args.parameter, args.numbilets));
    }   
}

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about)]
pub struct Config {
    #[arg(value_name = "FILE", help = "Input file(s)",default_value = "-")]
    files: Vec<String>,
    #[arg(short, long = "number", help = "Number lines", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,
    #[arg(short = 'b', long = "number-nonblank", help = "Number non-blank lines", conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let config = Config::parse();
    config.files.iter().try_for_each(|filename| {
        match open(filename) {
            Err(e) => {
                eprintln!("Failed to open {}: {}", filename, e);
                Ok(())
            }
            Ok(reader) => {
                let mut line_num = 0;
                reader.lines().enumerate().try_for_each(|(index, line)| -> MyResult<()> {
                    let line = line?;

                    if config.number_lines {
                        print!("{:>6}\t", index + 1);
                    } else if config.number_nonblank_lines && !line.trim().is_empty() {
                        line_num += 1;
                        print!("{:>6}\t", line_num);
                    }

                    println!("{}",  line);
                    Ok(())
                })
            }
        }
    })
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
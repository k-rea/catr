use clap::Parser;
use std::error::Error;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines to print
    #[arg(short, long = "number", default_value = "false")]
    number_lines: bool,

    /// Number non-block lines
    #[arg(short = 'b', long = "number-nonblock", default_value = "false")]
    number_nonblock_lines: bool,
}
pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    for file in args.files {
        match open(&file) {
            Err(e) => eprintln!("{file}: {e}"),
            Ok(file) => {
                let lines: Vec<_> = file.lines().collect::<Result<_, _>>()?;
                let mut empty_line_count = 0;
                for (i, line) in lines.iter().enumerate() {
                    if line.is_empty() && args.number_nonblock_lines {
                        empty_line_count += 1;
                    } else if args.number_lines || args.number_nonblock_lines {
                        print!("{:6}\t", i + 1 - empty_line_count);
                    }
                    print!("{}", line);
                    if i != lines.len() - 1 {
                        println!()
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}

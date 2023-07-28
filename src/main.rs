use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::Path;

use parser::parse;

mod error;
mod parser;
mod token;

fn main() {
    if let Ok(lines) = read_lines("rules.in") {
        let mut file = File::create("rules.out").unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("rules.out")
            .unwrap();

        for line in lines {
            if let Ok(rule) = line {
                if rule.len() != 0 {
                    let rule = parse(&rule).unwrap();
                    writeln!(file, "{rule:?}").unwrap();
                } else {
                    writeln!(file, "").unwrap();
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

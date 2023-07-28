use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Lines, Write};
use std::path::Path;

use parser::parse;

mod error;
mod parser;
mod token;

fn main() {
    if let Ok(lines) = read_lines("rules.in") {
        File::create("rules.rs").unwrap();
        let mut buffer = String::new();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("rules.rs")
            .unwrap();
        let mut rule_count = 0;
        
        let mut lines = lines;
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.starts_with("//") || line.len() == 0 {
                continue;
            }
            
            let from = parse(&line).unwrap();
            let line = lines.next().unwrap().unwrap();
            let to = parse(&line).unwrap();
            buffer.push_str(&format!("\n\t\tAxiom::new(\n\t\t\t{from},\n\t\t\t{to}\n\t\t),"));
            rule_count += 1;
        }
        
        buffer.push_str("\n\t]\n}");
        writeln!(file, "fn get_rules() -> [Rule; {rule_count}] {{\n\t[\n{buffer}").unwrap();
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

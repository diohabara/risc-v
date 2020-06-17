use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_rom_lines(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let instructions: Vec<&str> = ip
                    .split(|c| c == ' ' || c == ',')
                    .filter(|&s| s != "")
                    .collect();
                for inst in instructions {
                    print!("{} ", inst);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

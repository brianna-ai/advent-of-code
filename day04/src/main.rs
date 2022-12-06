use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// from rust-lang docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// part 1
fn fully_overlapping_assts(elf_one: Vec<&str>, elf_two: Vec<&str>) -> bool {
    let elf_one_start = elf_one[0].parse::<i32>().unwrap();
    let elf_one_end = elf_one[1].parse::<i32>().unwrap();
    let elf_two_start = elf_two[0].parse::<i32>().unwrap();
    let elf_two_end = elf_two[1].parse::<i32>().unwrap();
    match elf_one_start == elf_two_start {
        true => true, // if they start at the same section, one has to be contained within the other
        false => {
            match elf_one_start < elf_two_start {
                true => {
                    if elf_one_end >= elf_two_end {
                        true
                    } else {
                        false
                    }
                },
                false => {
                    if elf_one_end <= elf_two_end {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

// part 2
fn any_overlapping_assts(elf_one: Vec<&str>, elf_two: Vec<&str>) -> bool {
    let elf_one_start = elf_one[0].parse::<i32>().unwrap();
    let elf_one_end = elf_one[1].parse::<i32>().unwrap();
    let elf_two_start = elf_two[0].parse::<i32>().unwrap();
    let elf_two_end = elf_two[1].parse::<i32>().unwrap();
    match elf_one_start == elf_two_start {
        true => true, // if they start at the same section, one has to be contained within the other
        false => {
            match elf_one_start < elf_two_start {
                true => { // elf one cleaning sections before elf two starts
                    if elf_one_end >= elf_two_start { // elf one cleaning sections after elf two starts
                        true
                    } else {
                        false
                    }
                },
                false => {
                    if elf_two_end >= elf_one_start {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

// part 1: full overlap is 567
// part 2: any overlap is 907
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01

    let mut overlapped_assts_count = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let assts: Vec<&str> = ip.split(",").collect::<Vec<_>>();
                let elf_one: Vec<&str> = assts[0].split("-").collect::<Vec<_>>();
                let elf_two: Vec<&str> = assts[1].split("-").collect::<Vec<_>>();
                if any_overlapping_assts(elf_one, elf_two) {
                    overlapped_assts_count = overlapped_assts_count + 1;
                }
            }
        }
    }
    println!("{}", overlapped_assts_count);
}

//use std::collections::Vec;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn sum_calories(calories: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in calories {
        total = total + i;
    }
    total
}

// from rust-lang docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01
    let top_number = &args[2].parse::<i32>().unwrap(); // how many elves we want (1 for part 1, 3 for part 2)

    let mut elf_vec = Vec::new();
    let mut cur_calories = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let to_int = ip.parse::<i32>();
                if let Ok(int) = to_int {
                    cur_calories.push(int);
                } else {
                    // sum calories and update vec + index
                    let total = sum_calories(&cur_calories);
                    elf_vec.push(total);
                    // reset cur_calories vec
                    cur_calories.clear();
                }
            }
        }
        // last elf info
        // sum calories and update vec + index
        let total = sum_calories(&cur_calories);
        elf_vec.push(total);
        // reset cur_calories vec
        cur_calories.clear();
    }
    let mut top_elves = Vec::new();
    for _i in 0..*top_number {
        let top_cal_left = elf_vec.iter().max().unwrap().clone();
        top_elves.push(top_cal_left);
        let idx = elf_vec.iter().position(|&x| x == top_cal_left).unwrap();
        elf_vec.remove(idx);
    }
    println!("{}", sum_calories(&top_elves)); // 68442, 204837 for parts 1 and 2
}
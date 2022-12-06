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

fn rem_first(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.as_str().to_string()
}

fn found_marker(idx: usize, buf: String, marker_type: usize) -> (bool, usize) {
    for i in 0..marker_type {
        for j in i+1..marker_type {
            let cur_char = buf.chars().nth(i + idx).unwrap();
            let comp_char = buf.chars().nth(j + idx).unwrap();
            if cur_char == comp_char {
                return (false, 1 + idx);
            }
        }
    }
    (true, idx)
}

fn start_of_packet_marker(buf: String, marker_type: usize) -> usize {
    let mut base_idx = 0;
    let buf_len = buf.chars().count();
    while base_idx + marker_type < buf_len {
        let (found, idx) = found_marker(base_idx, buf.clone(), marker_type);
        if found {
            base_idx = base_idx + marker_type;
            break;
        }
        base_idx = idx;
    }
    base_idx
}

fn find_markers(buf: String) -> (usize, usize) {
    (start_of_packet_marker(buf.clone(), 4), start_of_packet_marker(buf.clone(), 14))
}

// this code is soooooo slowwwwww it's very bad, come back if you have time
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", find_markers(ip));
                dbg!(find_markers(ip));
            }
        }
    }
}

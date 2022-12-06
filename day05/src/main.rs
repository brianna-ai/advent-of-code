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

// get the stacks
fn get_stacks(mut crates: Vec<String>) -> Vec<Vec<String>> {
    let stack_names = crates.pop().unwrap();
    let stacks: Vec<&str> = stack_names.split("   ").collect::<Vec<_>>();
    let num_stacks = stacks.len();
    let mut stack_vecs: Vec<Vec<String>> = Vec::new();
    for _stack in stacks {
        stack_vecs.push(Vec::new());
    }
    for layer in crates.iter().rev() {
        let layer_fill_empty = layer.replace("    ", " ").to_string();
        let layer_as_vec: Vec<&str> = layer_fill_empty.split(" ").collect::<Vec<_>>();
        for i in 0..num_stacks {
            let cur_value = (&layer_as_vec[i]).to_string();
            if !cur_value.is_empty() {
                stack_vecs[i].push(cur_value);
            }
        }
    }
    stack_vecs
}

// stack
fn read_instrs_CM9000(mut stacks: Vec<Vec<String>>, instrs: Vec<String>) -> Vec<Vec<String>> {
    for instr in instrs {
        let instr_as_vec: Vec<&str> = instr.split(" ").collect::<Vec<_>>();
        let num_moved = instr_as_vec[1].parse::<usize>().unwrap();
        let src = instr_as_vec[3].parse::<usize>().unwrap() - 1;
        let dst = instr_as_vec[5].parse::<usize>().unwrap() - 1;
        for _i in 0..num_moved {
            let moving_crate = stacks[src].pop().unwrap();
            stacks[dst].push(moving_crate);
        }
    }
    stacks
}

// queue
fn read_instrs_CM9001(mut stacks: Vec<Vec<String>>, instrs: Vec<String>) -> Vec<Vec<String>> {
    for instr in instrs {
        let instr_as_vec: Vec<&str> = instr.split(" ").collect::<Vec<_>>();
        let num_moved = instr_as_vec[1].parse::<usize>().unwrap();
        let src = instr_as_vec[3].parse::<usize>().unwrap() - 1;
        let dst = instr_as_vec[5].parse::<usize>().unwrap() - 1;

        let src_num_crates = stacks[src].len();
        let moving_crates = stacks[src].split_off(src_num_crates - num_moved);
        stacks[dst].extend(moving_crates);
    }
    stacks
}

// get the top crates
fn scrape_top(mut stacks: Vec<Vec<String>>) -> Vec<String> {
    let mut top = Vec::new();
    for mut stack in stacks {
        top.push(stack.pop().unwrap());
    }
    top
}

// part 1: top crates after move by CrateMover9000 are BZLVHBWQF
// part 2: top crates after move by CrateMover9001 are TDGJQTZSL
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01

    let mut input = Vec::new();
    let mut stacks = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    stacks = get_stacks(input.clone());
                    input.clear();
                    continue;
                }
                input.push(ip);
            }
        }
    }
    // logic to read instructions
    //let final_stacks = read_instrs_CM9000(stacks, input);
    let final_stacks = read_instrs_CM9001(stacks, input);
    dbg!(scrape_top(final_stacks));
}

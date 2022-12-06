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

// return the shared item (char) between the two compartments
// assumed that compartments are the same length (if relevant)
fn shared_item(compartment_one: &[char], compartment_two: &[char]) -> Vec<char> {
    let mut shared = Vec::new();
    for item_one in compartment_one {
        for item_two in compartment_two {
            if item_one == item_two {
                shared.push(*item_one);
            }
        }
    }
    shared // should not reach here since we are guaranteed there is exactly one shared
}

fn get_priority(item: char) -> u32 {
    let mut priority = 0;
    if 'A' <= item && item <= 'Z' {
        // 27-52
        priority = (item as u32 - 'A' as u32) + 27;
    } else if 'a' <= item && item <= 'z' {
        // 1-26
        priority = (item as u32 - 'a' as u32) + 1;
    } 
    priority
}

fn sum_priorities(items: &Vec<char>) -> u32 {
    let mut total_priorities = 0; // sum of priorities
    for item in items {
        total_priorities = total_priorities + get_priority(*item);
    }
    total_priorities
}

// part 1: 7795
// part 2: 2703
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; // src/[file] if running from day01

    let mut item_vec = Vec::new(); // vector for storing shared items between compartments
    let mut elf_group = Vec::new(); // three rucksacks
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                /*/ START part 1
                // String to Vec<char>
                let chars: Vec<char> = ip.chars().collect::<Vec<_>>();
                let compartment_size = chars.len() / 2;
                let compartments: Vec<&[char]> = chars.chunks(compartment_size).collect();
                item_vec.push(shared_item(&compartments[0], &compartments[1])[0]);
                // END part 1 / */
                let chars: Vec<char> = ip.chars().collect::<Vec<_>>();
                elf_group.push(chars);

                if elf_group.len() == 3 {
                    // considering groups of three for part 2
                    let shared_so_far = shared_item(&elf_group[0], &elf_group[1]);
                    item_vec.push(shared_item(&shared_so_far, &elf_group[2])[0]);
                    // clear when finished
                    elf_group.clear();
                }
            }
        }
    }
    // dbg!(item_vec);
    // dbg!(elf_group);
    println!("{}", sum_priorities(&item_vec));
}

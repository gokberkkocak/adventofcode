use std::collections::HashSet;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2018,1);
    let mut sum : isize = 0;
    let mut seen : HashSet<isize> = HashSet::new();
    let mut freq_changes : Vec<isize> = Vec::new();
    for line in input.lines(){
        let freq : isize = line.parse().unwrap();
        freq_changes.push(freq);
    }
    let mut flag = true;
    'big:loop {
        for i in 0..freq_changes.len(){
            seen.insert(sum);
            sum = sum + freq_changes[i];
            if seen.contains(&sum){
                println!("part 2 sum {}", sum);
                break 'big;
            }
        }
        if flag {
            println!("part 1 sum {}", sum);
            flag = false;
        }
    }
   // println!("{:?}", seen);
}

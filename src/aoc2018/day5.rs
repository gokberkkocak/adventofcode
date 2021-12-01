

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2018,5);
    let mut v = Vec::new();
    for c in b'a'..=b'z' {
        let mut output = "".to_string();
        let input_modified = input
            .chars()
            .filter(|&d| char::from(c) != d && char::from(c).to_ascii_uppercase() != d);
        for i in input_modified {
            if i != output.chars().last().unwrap_or('0')
                && i.to_ascii_uppercase()
                    == output.chars().last().unwrap_or('0').to_ascii_uppercase()
            {
                output.pop();
            } else {
                output.push(i);
            }
        }
        v.push(output.len());
        println!("p1 {}", output.len());
    }
    println!("p2 {}", v.iter().min().unwrap());
}
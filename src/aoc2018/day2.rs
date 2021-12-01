use crate::util::get_puzzle_input;

pub fn run()  {
	part1().unwrap();
	part2().unwrap();
}

fn part2() -> std::io::Result<()> {
	let input = get_puzzle_input(2018, 2);
	let mut boxes : Vec<String> = Vec::new(); 
	for line in input.lines() {
		boxes.push(line.to_string());
	}
	let mut max_str : String = "".to_string(); 
	let mut max = 0;
	let mut max_locs : Vec<usize> = Vec::new();
	for b1 in 0..boxes.len(){
		for b2 in b1+1..boxes.len(){
			let b1_chars : Vec<char> = boxes[b1].chars().collect();
			let b2_chars : Vec<char> = boxes[b2].chars().collect();
			let mut count = 0;
			let mut diff_loc = Vec::new();
			for i in 0..boxes[b1].len(){
				if b1_chars[i] == b2_chars[i]{
					count += 1;
				}
				else{
					diff_loc.push(i);
				}
			}
			//println!("{} {} {}", b1, b2, count);			
			if count > max {
				max_str = boxes[b1].clone();
				max = count;
				max_locs = diff_loc.clone();
			}
		}

	}
	//println!("max sim {}", max);
	//println!("for {}", max_str);
	//println!("diff {:?}", max_locs);
	//println!("------");
	let mut str_chars : Vec<char> = max_str.chars().collect();
	for i in max_locs.iter(){
		str_chars.remove(*i);
	}
	let final_str : String = str_chars.iter().collect();
	println!("part 2 str {}", final_str);
	Ok(())
}


fn part1() -> std::io::Result<()> {
	let input = get_puzzle_input(2018, 2);
	let mut total_two = 0;
	let mut total_three = 0;
	for line in input.lines() {
		let mut seen_once: Vec<char> = Vec::new();
		let mut seen_twice: Vec<char> = Vec::new();
		let mut two = 0;
		let mut three = 0;
		for c in line.chars() {
			if seen_twice.contains(&c) {
				three += 1;
				two -= 1;
			} else if seen_once.contains(&c) {
				seen_twice.push(c);
				two += 1;
			} else {
				seen_once.push(c);
			}
		}
		//println!("two {}, three {}", two, three);
		if two > 0 {
			total_two += 1;
		}
		if three > 0 {
			total_three += 1;
		}
	}
	println!("part 1 checksum {}", total_two * total_three);
	Ok(())
}

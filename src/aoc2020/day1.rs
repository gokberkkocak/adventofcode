use std::collections::HashSet;

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 1);
    let mut nums = vec![];
    input
        .lines()
        .for_each(|line| nums.push(line.parse().unwrap()));
    let target = 2020u32;
    let (x, y) = two_sum(&nums, target);
    println!("p1 {}", x * y);
    let v = three_sum(&mut nums, target);
    println!("p2 {}", v.iter().product::<u32>());
}

fn two_sum(nums: &Vec<u32>, target: u32) -> (u32, u32) {
    let mut complements = HashSet::new();
    for i in nums.iter() {
        let c = target - *i;
        if complements.contains(&c) {
            return (c, *i);
        } else {
            complements.insert(i);
        }
    }
    unreachable!()
}

pub fn three_sum(nums: &mut Vec<u32>, target: u32) -> [u32; 3] {
    nums.sort();
    for (i, &x) in nums.iter().enumerate() {
        let sub_target = target - x;
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[l] + nums[r];
            if sum > sub_target {
                r -= 1;
            } else if sum < sub_target {
                l += 1;
            } else {
                return [x, nums[l], nums[r]];
            }
        }
    }
    unreachable!()
}

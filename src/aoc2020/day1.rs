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

fn two_sum(nums: &[u32], target: u32) -> (u32, u32) {
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

pub fn three_sum(nums: &mut [u32], target: u32) -> [u32; 3] {
    nums.sort_unstable();
    for (i, &x) in nums.iter().enumerate() {
        let sub_target = target - x;
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums[l] + nums[r];
            match sum.cmp(&sub_target) {
                std::cmp::Ordering::Equal => return [x, nums[l], nums[r]],
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Greater => r -= 1,
            }
        }
    }
    unreachable!()
}

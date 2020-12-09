use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2020, 9);
    let numbers = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<isize>>();
    let p1 = part1(&numbers);
    println!("{}", p1);
    let p2 = part2(&numbers, p1);
    println!("{}", p2);
}
fn part1(numbers: &Vec<isize>) -> isize {
    let mut sum = numbers.iter().take(25).sum::<isize>();
    for ((l, &x), (r, &y)) in numbers
        .iter()
        .enumerate()
        .zip(numbers.iter().enumerate().skip(25))
    {
        if two_sum(&numbers[l..=r], y) {
            sum = sum + y - x;
        } else {
            return y;
        }
    }
    unreachable!()
}

fn part2(numbers: &Vec<isize>, invalid: isize) -> isize {
    let mut sum: u64 = 0;
    let summed_nums = numbers
        .iter()
        .map(|&x| {
            sum += x as u64;
            sum
        })
        .collect::<Vec<u64>>();
    for (l, &sum_x) in summed_nums.iter().enumerate() {
        for (r, &sum_y) in summed_nums.iter().enumerate().skip(l) {
            if sum_y - sum_x == invalid as u64 {
                let max = numbers[l + 1..=r].iter().max().unwrap();
                let min = numbers[l + 1..=r].iter().min().unwrap();
                return max + min;
            }
        }
    }
    unreachable!()
}

fn two_sum(nums: &[isize], target: isize) -> bool {
    for i in nums.iter() {
        let c = target - *i;
        if nums.contains(&c) {
            return true;
        }
    }
    false
}

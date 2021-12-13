use std::collections::HashMap;

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 15);
    let vec = parse(&input);
    let p1 = part1(&vec);
    println!("p1 {}", p1);
    let p2 = part2(&vec);
    println!("p2 {}", p2);
}

#[inline]
fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(vec: &[usize]) -> usize {
    calculate_with_map(vec, 2020)
}

fn part2(vec: &[usize]) -> usize {
    calculate_with_vec(vec, 30000000)
}

fn calculate_with_map(vec: &[usize], n: usize) -> usize {
    let mut map = HashMap::new();
    vec.iter().enumerate().for_each(|(i, &x)| {
        map.insert(x, i);
    });
    let mut key = *vec.iter().last().unwrap();
    for i in vec.len() - 1..n - 1 {
        let mut old_position = usize::MAX;
        map.entry(key)
            .and_modify(|x| {
                old_position = *x;
                *x = i;
            })
            .or_insert(i);
        let new_key = i.saturating_sub(old_position); 
        key = new_key; // set new key
    }
    key
}

fn calculate_with_vec(vec: &[usize], n: usize) -> usize {
    let mut big_vec = vec![usize::MAX; n];
    vec.iter().enumerate().for_each(|(i, &x)| big_vec[x] = i);
    let mut key = *vec.iter().last().unwrap();
    for i in vec.len() - 1..n - 1 {
        let mut loc = i; // new loc to write
        std::mem::swap(&mut loc, &mut big_vec[key]); // new loc becomes old loc
        key = i.saturating_sub(loc); // adjust key - if old loc is usize::MAX, saturating_sub will give 0.
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let vec = vec![0, 3, 6];
        let p1 = part1(&vec);
        assert_eq!(436, p1);
    }

    #[test]
    fn test_2() {
        let vec = vec![0, 3, 6];
        let p2 = part2(&vec);
        assert_eq!(175594, p2);
    }
}

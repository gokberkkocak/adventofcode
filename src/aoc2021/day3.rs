pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 3);
    let (v, bit_len) = parse(&input);
    let p1 = part1(&v, bit_len);
    println!("p1: {}", p1);
    let p2 = part2(v, bit_len);
    println!("p2: {}", p2);
}

fn parse(input: &str) -> (Vec<usize>, usize) {
    let bit_len = input.lines().next().unwrap().len();
    let v = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();
    (v, bit_len)
}

fn part1(v: &[usize], bit_len: usize) -> usize {
    let input_len = v.len();
    let g = v
        .iter()
        .fold(vec![0; bit_len], |acc, &bits| {
            acc.iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .iter()
        .enumerate()
        .map(|(i, &b)| ((b >= input_len / 2) as usize) << i)
        .sum::<usize>();
    let e = !g & ((1 << bit_len) - 1);
    e * g
}
fn part2(v: Vec<usize>, bit_len: usize) -> usize {
    let oxy = (0..bit_len)
        .rev()
        .scan(v.clone(), |oxy, i| {
            let flag = oxy.iter().filter(|&n| n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            remove_unworthy(oxy, i, !flag);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..bit_len)
        .rev()
        .scan(v, |co2, i| {
            let flag = co2.iter().filter(|&n| n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            remove_unworthy(co2, i, flag);
            co2.first().copied()
        })
        .last()
        .unwrap();

    oxy * co2
}

fn remove_unworthy(vec: &mut Vec<usize>, i: usize, flag: bool) {
    let mut j = 0;
    while j < vec.len() {
        if (vec[j] & 1 << i > 0) == flag {
            let _val = vec.remove(j);
        } else {
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_3_test.in");
        let (v, bit_len) = parse(&input);
        let p1 = part1(&v, bit_len);
        assert_eq!(p1, 198);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_3_test.in");
        let (v, bit_len) = parse(&input);
        let p2 = part2(v, bit_len);
        assert_eq!(p2, 230);
    }
}

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 7);
    let mut array = parse(&input);
    array.sort();
    let p1 = part1(&array);
    println!("Part 1: {}", p1);
    let p2 = part2(&array);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> Vec<isize> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

fn median(array: &[isize]) -> isize {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        ((array[ind_left] + array[ind_right]) / 2) as isize
    } else {
        array[(array.len() / 2)]
    }
}

fn mean(array: &[isize]) -> isize {
    (array.iter().sum::<isize>() as f64 / array.len() as f64).round() as isize
}

fn part1(sorted_array: &[isize]) -> isize {
    let median = median(sorted_array);
    sorted_array.iter().map(|x| (x - median).abs()).sum()
}

fn part2(array: &[isize]) -> isize {
    let mean = mean(array);
    array
        .iter()
        .map(|x| {
            let diff = (x - mean).abs();
            diff * (diff + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_7_test.in");
        let mut array = parse(&input);
        array.sort();
        let p1 = part1(&array);
        assert_eq!(p1, 37);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_7_test.in");
        let array = parse(&input);
        let p1 = part2(&array);
        assert_eq!(p1, 168);
    }
}

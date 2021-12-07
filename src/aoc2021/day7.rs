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

fn median(sorted_array: &[isize]) -> isize {
    if (sorted_array.len() % 2) == 0 {
        let ind_left = sorted_array.len() / 2 - 1;
        let ind_right = sorted_array.len() / 2;
        ((sorted_array[ind_left] + sorted_array[ind_right]) / 2) as isize
    } else {
        sorted_array[(sorted_array.len() / 2)]
    }
}

// apparently rounded mean might be still off so try get both floor and ceil.
fn around_mean(array: &[isize]) -> (isize, isize) {
    (
        (array.iter().sum::<isize>() as f64 / array.len() as f64).floor() as isize,
        (array.iter().sum::<isize>() as f64 / array.len() as f64).ceil() as isize,
    )
}

fn part1(sorted_array: &[isize]) -> isize {
    let median = median(sorted_array);
    sorted_array.iter().map(|x| (x - median).abs()).sum()
}

fn part2(array: &[isize]) -> isize {
    let values = around_mean(array);
    part2_core(&array, values.0).min(part2_core(&array, values.1))
}

fn part2_core(array: &[isize], value: isize) -> isize {
    array
        .iter()
        .map(|x| {
            let diff = (x - value).abs();
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

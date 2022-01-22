pub fn run() {
    let input = crate::util::get_puzzle_input(2018, 14);
    let p1 = part1(&input);
    println!("Part 1: {}", p1);
    let p2 = part2(&input);
    println!("Part 2: {}", p2);
}

struct Recipes {
    scores: Vec<usize>,
    elf1: usize,
    elf2: usize,
}

impl Recipes {
    fn next(&mut self) -> bool {
        let sum = self.scores[self.elf1] + self.scores[self.elf2];
        if sum >= 10 {
            self.scores.push(sum / 10);
        }
        self.scores.push(sum % 10);
        self.elf1 = (self.elf1 + self.scores[self.elf1] + 1) % self.scores.len();
        self.elf2 = (self.elf2 + self.scores[self.elf2] + 1) % self.scores.len();
        sum >= 10
    }
}

fn part1(input: &str) -> String {
    let n = input.parse::<usize>().unwrap();
    let mut recipes = Recipes {
        scores: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };
    while recipes.scores.len() < n + 10 {
        recipes.next();
    }
    recipes
        .scores
        .iter()
        .skip(n)
        .take(10)
        .map(|&x| char::from_digit(x as u32, 10).unwrap())
        .collect()
}

fn part2(input: &str) -> usize {
    let seq = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut recipes = Recipes {
        scores: vec![3, 7],
        elf1: 0,
        elf2: 1,
    };
    let seq_len = seq.len();
    loop {
        let double_add = recipes.next();
        let recipes_len = recipes.scores.len();
        if recipes_len > seq_len && recipes.scores[recipes_len - seq_len..recipes_len] == seq {
            return recipes_len - seq_len;
        }
        if double_add
            && recipes_len > seq_len + 1
            && recipes.scores[recipes_len - 1 - seq_len..recipes_len - 1] == seq
        {
            return recipes_len - 1 - seq_len;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let p1 = part1("9");
        assert_eq!(p1, "5158916779");
    }

    #[test]
    fn test_1_2() {
        let p1 = part1("5");
        assert_eq!(p1, "0124515891");
    }

    #[test]
    fn test_1_3() {
        let p1 = part1("18");
        assert_eq!(p1, "9251071085");
    }

    #[test]
    fn test_1_4() {
        let p1 = part1("2018");
        assert_eq!(p1, "5941429882");
    }

    #[test]
    fn test_2_1() {
        let p2 = part2("51589");
        assert_eq!(p2, 9);
    }

    #[test]
    fn test_2_2() {
        let p2 = part2("01245");
        assert_eq!(p2, 5);
    }

    #[test]
    fn test_2_3() {
        let p2 = part2("92510");
        assert_eq!(p2, 18);
    }

    #[test]
    fn test_2_4() {
        let p2 = part2("59414");
        assert_eq!(p2, 2018);
    }
}

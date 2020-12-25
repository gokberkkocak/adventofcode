static DIVIDER: usize = 20201227;
static SUBJECT: usize = 7;

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 25);
    let p1 = part1(&input);
    println!("p1 {}", p1);
}

fn parse(input: &str) -> (usize, usize) {
    let v = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    (v[0], v[1])
}

fn get_loop_size(pub_key: usize) -> usize {
    let mut value = 1;
    let mut card_loop = 0;
    while value != pub_key {
        value = value * SUBJECT % DIVIDER;
        card_loop += 1;
    }
    card_loop
}

fn get_encryption(pub_key: usize, other_loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..other_loop_size {
        value = value * pub_key % DIVIDER;
    }
    value
}

fn part1(input: &str) -> usize {
    let (card_pub, door_pub) = parse(&input);
    let dl = get_loop_size(door_pub);
    let enc = get_encryption(card_pub, dl);
    enc
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let card_pub = 5764801;
        let door_pub = 17807724;
        let cl = get_loop_size(card_pub);
        let dl = get_loop_size(door_pub);
        assert_eq!(cl, 8);
        assert_eq!(dl, 11);
        let enc = get_encryption(card_pub, dl);
        let enc_2 = get_encryption(door_pub, cl);
        assert_eq!(enc, 14897079);
        assert_eq!(enc, enc_2);
    }
}

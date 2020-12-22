use fnv::FnvHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 22);
    let mut game = parse(&input);
    let mut cloned = game.clone();
    let part_1 = part1(&mut cloned);
    println!("{}", part_1);
    let part_2 = part2(&mut game);
    println!("{}", part_2);
}

fn parse(input: &str) -> GameBoard {
    let mut v = vec![];
    for raw_p in input.split("\n\n") {
        let p = raw_p
            .lines()
            .skip(1)
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<VecDeque<_>>();
        v.push(Player::new(p));
    }
    let mut v_it = v.into_iter();
    let p1 = v_it.next().unwrap();
    let p2 = v_it.next().unwrap();
    GameBoard::new(p1, p2)
}

fn part1(game: &mut GameBoard) -> usize {
    game.play_combat();
    game.calculate_score()
}

fn part2(game: &mut GameBoard) -> usize {
    game.play_recursive_combat();
    game.calculate_score()
}

#[derive(Debug, Clone)]
struct Player {
    cards: VecDeque<u8>,
}

impl Player {
    fn new(cards: VecDeque<u8>) -> Self {
        Self { cards }
    }
}

#[derive(Debug, Clone)]
struct GameBoard {
    p1: Player,
    p2: Player,
    prev_rounds: HashSet<u64>,
}

impl GameBoard {
    fn new(p1: Player, p2: Player) -> Self {
        Self {
            p1,
            p2,
            prev_rounds: HashSet::new(),
        }
    }

    fn play_combat(&mut self) {
        while !self.is_combat_game_over() {
            let p1_card = self.p1.cards.pop_front().unwrap();
            let p2_card = self.p2.cards.pop_front().unwrap();
            if p1_card > p2_card {
                self.p1.cards.extend(&[p1_card, p2_card]);
            } else {
                self.p2.cards.extend(&[p2_card, p1_card]);
            }
        }
    }

    fn play_recursive_combat(&mut self) -> usize {
        while !self.is_combat_game_over() {
            if !self.add_to_seen_rounds() {
                return 0;
            }
            let p1_card = self.p1.cards.pop_front().unwrap() as usize;
            let p2_card = self.p2.cards.pop_front().unwrap() as usize;
            let round_winner = if p1_card <= self.p1.cards.len() && p2_card <= self.p2.cards.len() {
                let sub_p1 = Player::new(self.p1.cards.iter().take(p1_card).copied().collect());
                let sub_p2 = Player::new(self.p2.cards.iter().take(p2_card).copied().collect());
                let mut sub_game = GameBoard::new(sub_p1, sub_p2);
                sub_game.play_recursive_combat()
            } else {
                if p1_card > p2_card {
                    0
                } else {
                    1
                }
            };
            if round_winner == 0 {
                self.p1.cards.extend(&[p1_card as u8, p2_card as u8]);
            } else {
                self.p2.cards.extend(&[p2_card as u8, p1_card as u8]);
            }
        }
        self.get_winner()
    }

    fn is_combat_game_over(&self) -> bool {
        self.p1.cards.is_empty() || self.p2.cards.is_empty()
    }

    fn calculate_hashes(&self) -> u64 {
        let mut hasher = FnvHasher::default();
        self.p1.cards.hash(&mut hasher);
        self.p2.cards.hash(&mut hasher);
        hasher.finish()
    }

    fn add_to_seen_rounds(&mut self) -> bool {
        let hash = self.calculate_hashes();
        self.prev_rounds.insert(hash)
    }

    fn get_winner(&self) -> usize {
        match (self.p1.cards.is_empty(), self.p2.cards.is_empty()) {
            (true, true) => unreachable!(),
            (true, false) => 1,
            (false, true) => 0,
            (false, false) => panic!("Please don't call me if the game is not done"),
        }
    }

    fn get_winner_player(&self) -> &Player {
        if self.get_winner() == 0 {
            &self.p1
        } else {
            &self.p2
        }
    }

    fn calculate_score(&self) -> usize {
        self.get_winner_player()
            .cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (i + 1) * c as usize)
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_22_test.in");
        let mut game = parse(&input);
        let p1 = part1(&mut game);
        assert_eq!(p1, 306);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_22_test.in");
        let mut game = parse(&input);
        let p2 = part2(&mut game);
        assert_eq!(p2, 291);
    }
}

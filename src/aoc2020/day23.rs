pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 23);
    let p1 = part1(&input);
    println!("p1 {}", p1);
    let p2 = part2(&input);
    println!("p2 {}", p2);
}

fn part1(input: &str) -> String {
    let mut cup_game = Game::new(input);
    cup_game.play_game(100);
    cup_game.get_string_representation()
}

fn part2(input: &str) -> usize {
    let mut cup_game = Game::new_with_extra(input);
    cup_game.play_game(10000000);
    cup_game.get_score()
}

struct Game {
    cups: Vec<usize>,
    current_cup: usize,
}

impl Game {
    fn new(input: &str) -> Self {
        let cups = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        Game {
            cups: to_linked_vec(&cups),
            current_cup: cups[0],
        }
    }

    fn new_with_extra(input: &str) -> Self {
        let mut cups = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        cups.extend(cups.iter().copied().max().unwrap() + 1..=1000000);
        Game {
            cups: to_linked_vec(&cups),
            current_cup: cups[0],
        }
    }

    fn play_round(&mut self) {
        let mut dest_cup = if self.current_cup == 1 {
            self.cups.len() - 1
        } else {
            self.current_cup - 1
        };
        let picked_1 = self.cups[self.current_cup]; // take 3
        let picked_2 = self.cups[picked_1];
        let picked_3 = self.cups[picked_2];
        while [picked_1, picked_2, picked_3].contains(&dest_cup) {
            dest_cup = if dest_cup == 1 {
                self.cups.len() - 1
            } else {
                dest_cup - 1
            };
        }
        //adjust
        self.cups[self.current_cup] = self.cups[picked_3]; // current now shows one after 3rd pick
        let post_dest = self.cups[dest_cup]; // adjusting where dest shows
        self.cups[dest_cup] = picked_1;
        self.cups[picked_1] = picked_2;
        self.cups[picked_2] = picked_3;
        self.cups[picked_3] = post_dest;
        self.current_cup = self.cups[self.current_cup];
    }

    fn play_game(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.play_round();
        }
    }

    fn get_string_representation(&self) -> String {
        let mut s = String::new();
        let mut i = self.cups[1];
        while i != 1 {
            s.push_str(&i.to_string());
            i = self.cups[i];
        }
        s
    }

    fn get_score(&self) -> usize {
        let el1 = self.cups[1];
        let el2 = self.cups[el1];
        el1 * el2
    }
}

fn to_linked_vec(cups: &[usize]) -> Vec<usize> {
    let mut cups_linked = vec![0; cups.len() + 1];
    for window in cups.windows(2) {
        cups_linked[window[0]] = window[1];
    }
    cups_linked[cups[cups.len() - 1]] = cups[0];
    cups_linked
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = "389125467";
        let mut cup_game = Game::new(input);
        cup_game.play_game(10);
        let s = cup_game.get_string_representation();
        assert_eq!(s, "92658374");
    }
    #[test]
    fn test_2() {
        let input = "389125467";
        let s = part1(input);
        assert_eq!(s, "67384529");
    }

    #[test]
    fn test_3() {
        let input = "389125467";
        let score = part2(input);
        assert_eq!(score, 149245887792);
    }
}

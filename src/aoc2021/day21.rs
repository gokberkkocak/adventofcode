const PRACTICE_WIN_SCORE: usize = 1000;
const QUANTUM_WIN_SCORE: usize = 21;

use fxhash::FxHashMap;
use itertools::Itertools;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 21);
    let game = parse(&input);
    let p1 = game.practice_play();
    println!("Part 1: {}", p1);
    let p2 = game.quantum_play();
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> DiracDieGame {
    let mut it = input.lines();
    let p1_starting = it
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let p2_starting = it
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    DiracDieGame::new(p1_starting, p2_starting)
}

struct DiracDieGame {
    p1_starting: usize,
    p2_starting: usize,
}

impl DiracDieGame {
    fn new(p1_starting: usize, p2_starting: usize) -> Self {
        DiracDieGame {
            p1_starting,
            p2_starting,
        }
    }

    #[inline]
    fn practice_play(&self) -> usize {
        let mut it = (1..=100).cycle();
        let mut p1 = self.p1_starting;
        let mut p2 = self.p2_starting;
        let mut p1_score = 0;
        let mut p2_score = 0;
        let mut dice_count = 0;
        loop {
            Self::practice_inner_play(&mut p1, &mut it, &mut p1_score, &mut dice_count);
            if p1_score >= PRACTICE_WIN_SCORE {
                return p2_score * dice_count;
            }
            Self::practice_inner_play(&mut p2, &mut it, &mut p2_score, &mut dice_count);
            if p2_score >= PRACTICE_WIN_SCORE {
                return p1_score * dice_count;
            }
        }
    }
    #[inline]
    fn practice_inner_play(
        p: &mut usize,
        it: &mut impl Iterator<Item = usize>,
        score: &mut usize,
        dice_count: &mut usize,
    ) {
        *p = (it.take(3).sum::<usize>() + *p - 1) % 10 + 1;
        *score += *p;
        *dice_count += 3;
    }

    #[inline]
    fn quantum_play(&self) -> usize {
        // pre-compute 27 combinations and count the number of occurrences for each outcome as well.
        let outcomes_occ = (1..=3)
            .cartesian_product(1..=3)
            .cartesian_product(1..=3)
            .map(|((d1, d2), d3)| d1 + d2 + d3)
            .fold(FxHashMap::<usize, usize>::default(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            })
            .into_iter()
            .collect_vec();
        let mut cache = FxHashMap::default();
        let (p1_win, p2_win) = Self::quantum_inner_play(
            &mut cache,
            &outcomes_occ,
            self.p1_starting,
            0,
            self.p2_starting,
            0,
        );
        p1_win.max(p2_win)
    }

    fn quantum_inner_play(
        cache: &mut FxHashMap<(usize, usize, usize, usize), (usize, usize)>,
        outcomes_occ: &[(usize, usize)],
        cur_pos: usize,
        cur_score: usize,
        other_pos: usize,
        other_score: usize,
    ) -> (usize, usize) {
        // check other win condition first
        if other_score >= QUANTUM_WIN_SCORE {
            return (0, 1);
        }
        // check cache
        if let Some(&score) = cache.get(&(cur_pos, cur_score, other_pos, other_score)) {
            return score;
        }
        let mut cur_win_count = 0;
        let mut oth_win_count = 0;
        for (outcome, occ) in outcomes_occ {
            let cur_pos = (cur_pos + outcome - 1) % 10 + 1;
            let cur_score = cur_score + cur_pos;
            let (oth_win_add, cur_win_add) = Self::quantum_inner_play(
                cache,
                outcomes_occ,
                other_pos,
                other_score,
                cur_pos,
                cur_score,
            );
            oth_win_count += oth_win_add * occ;
            cur_win_count += cur_win_add * occ;
        }
        // add to cache
        cache.insert(
            (cur_pos, cur_score, other_pos, other_score),
            (cur_win_count, oth_win_count),
        );
        (cur_win_count, oth_win_count)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_21_test.in");
        let game = parse(&input);
        let p1 = game.practice_play();
        assert_eq!(p1, 739785);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_21_test.in");
        let game = parse(&input);
        let p1 = game.quantum_play();
        assert_eq!(p1, 444356092776315);
    }
}

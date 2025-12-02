use std::collections::HashMap;

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 21);
    let (mut pool, words) = parse(&input);
    pool.solve();
    let p1 = part1(&pool, &words);
    println!("p1 {}", p1);
    let p2 = part2(&pool);
    println!("p2 {}", p2);
}

fn parse(input: &str) -> (AllergenPool<'_>, Vec<&str>) {
    let mut ingredient_domains: HashMap<(usize, usize), AllergenSupport> = HashMap::new();
    let mut allergen_set = vec![];
    let mut ingredient_set = vec![];
    let mut ing_vec = vec![];
    for line in input.lines() {
        let mut it = line.split(" (contains ");
        let ingredients = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<_>>();
        let allergens = it
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();
        for i in ingredients {
            ing_vec.push(i);
            for a in allergens.clone() {
                if !ingredient_set.contains(&i) {
                    ingredient_set.push(i);
                }
                if !allergen_set.contains(&a) {
                    allergen_set.push(a);
                }
                let i_id = ingredient_set.iter().position(|&x| x == i).unwrap();
                let a_id = allergen_set.iter().position(|&x| x == a).unwrap();
                ingredient_domains
                    .entry((i_id, a_id))
                    .and_modify(|e| e.increment(1))
                    .or_insert_with(|| AllergenSupport::new(i_id, a_id));
            }
        }
    }
    (
        AllergenPool::new(ingredient_domains, ingredient_set, allergen_set),
        ing_vec,
    )
}

fn part1(pool: &AllergenPool, words: &[&str]) -> usize {
    words
        .iter()
        .cloned()
        .filter(|&x| {
            let id = pool.ingredient_set.iter().position(|&i| i == x).unwrap();
            !pool.assignments.contains_key(&id)
        })
        .count()
}

fn part2(pool: &AllergenPool) -> String {
    let mut sorted_allergens = pool.allergen_set.clone();
    sorted_allergens.sort_unstable();
    let mut v = vec![];
    for i in sorted_allergens {
        let a_id = pool.allergen_set.iter().position(|&x| x == i).unwrap();
        let i_id = *pool
            .assignments
            .iter()
            .find(|(_i, &a)| a_id == a)
            .unwrap()
            .0;
        v.push(pool.ingredient_set[i_id]);
    }
    let mut res = String::new();
    for i in v {
        res.push_str(i);
        res.push(',');
    }
    res.strip_suffix(',').unwrap().to_string()
}

#[derive(Debug)]
struct AllergenPool<'a> {
    ingredient_domains: HashMap<(usize, usize), AllergenSupport>,
    ingredient_set: Vec<&'a str>,
    allergen_set: Vec<&'a str>,
    assignments: HashMap<usize, usize>,
}

impl<'a> AllergenPool<'a> {
    fn new(
        ingredient_domains: HashMap<(usize, usize), AllergenSupport>,
        ingredient_set: Vec<&'a str>,
        allergen_set: Vec<&'a str>,
    ) -> Self {
        Self {
            ingredient_domains,
            ingredient_set,
            allergen_set,
            assignments: HashMap::new(),
        }
    }
    fn solve(&mut self) {
        fn inner_solve<'a>(
            ingredient_domains: &mut HashMap<(usize, usize), AllergenSupport>,
            assignments: &mut HashMap<usize, usize>,
            ingredient_set: &[&'a str],
            allergen_set: &[&'a str],
        ) -> bool {
            let max_sup = ingredient_domains
                .iter()
                .map(|((_i, _a), x)| x.support)
                .max()
                .unwrap();
            let max_keys = ingredient_domains
                .iter()
                .filter(|&((_i, _a), t)| t.support == max_sup)
                .map(|(&(i, a), _)| (i, a))
                .collect::<Vec<_>>();
            if allergen_set.len() == assignments.len() {
                return true;
            }
            for (ing, allerg) in max_keys {
                if assignments.contains_key(&ing) {
                    continue;
                }
                assignments.insert(ing, allerg);
                let times = ingredient_domains.get(&(ing, allerg)).unwrap().support;
                for (i_id, _i) in ingredient_set.iter().enumerate() {
                    ingredient_domains
                        .entry((i_id, allerg))
                        .and_modify(|e| e.decrement(times));
                }
                if inner_solve(
                    ingredient_domains,
                    assignments,
                    ingredient_set,
                    allergen_set,
                ) {
                    return true;
                }
                // revert
                else {
                    for (i_id, _i) in ingredient_set.iter().enumerate() {
                        if ingredient_domains.contains_key(&(i_id, allerg)) {
                            ingredient_domains
                                .entry((i_id, allerg))
                                .and_modify(|e| e.increment(times));
                        }
                    }
                    assignments.remove(&ing);
                }
            }
            false
        }
        let mut assignments = HashMap::new();
        if inner_solve(
            &mut self.ingredient_domains,
            &mut assignments,
            &self.ingredient_set,
            &self.allergen_set,
        ) {
            self.assignments = assignments;
        }
    }
}

#[derive(Debug)]
struct AllergenSupport {
    _ingredient_id: usize,
    _allergen_id: usize,
    support: isize, // this lad can go negative and it is necessary for backtracking
}

impl AllergenSupport {
    fn new(ingredient_id: usize, allergen_id: usize) -> Self {
        Self {
            _ingredient_id: ingredient_id,
            _allergen_id: allergen_id,
            support: 1,
        }
    }

    fn increment(&mut self, value: isize) {
        self.support += value;
    }

    fn decrement(&mut self, value: isize) {
        self.support -= value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_21_test.in");
        let (mut pool, words) = parse(&input);
        pool.solve();
        let p1 = part1(&pool, &words);
        assert_eq!(p1, 5);
    }
    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2020_21_test.in");
        let (mut pool, _) = parse(&input);
        pool.solve();
        let p2 = part2(&pool);
        assert_eq!(p2, "mxmxvkd,sqjhc,fvjkl");
    }
}

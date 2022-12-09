pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 8);
    let mut forest = parse(&input);
    forest.calculate_vis();
    let p1 = part1(&forest);
    println!("Part 1: {}", p1);
    let p2 = part2(&forest);
    println!("Part 2: {}", p2);
}

fn part1(forest: &Forest) -> usize {
    forest
        .visibility
        .iter()
        .flatten()
        .filter(|x| **x > 0)
        .count()
}

fn part2(forest: &Forest) -> usize {
    forest
        .visibility
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, vis)| **vis > 0)
                .filter(move |(x, _)| x > &0 && x < &(row.len() - 1)) // not on the edge
                .filter(move |_| y > 0 && y < forest.trees.len() - 1) // not on the edge
                .map(move |(x, _)| {
                    let mut scene_score = 1;
                    // from the left
                    for (count, i) in (0..x).rev().enumerate() {
                        if forest.trees[y][i] >= forest.trees[y][x] || i == 0 {
                            scene_score *= count + 1;
                            break;
                        }
                    }
                    // from the right
                    for (count, i) in (x + 1..forest.trees[0].len()).enumerate() {
                        if forest.trees[y][i] >= forest.trees[y][x]
                            || i == forest.trees[0].len() - 1
                        {
                            scene_score *= count + 1;
                            break;
                        }
                    }
                    // from the top
                    for (count, i) in (0..y).rev().enumerate() {
                        if forest.trees[i][x] >= forest.trees[y][x] || i == 0 {
                            scene_score *= count + 1;
                            break;
                        }
                    }
                    // from the bottom
                    for (count, i) in (y + 1..forest.trees.len()).enumerate() {
                        if forest.trees[i][x] >= forest.trees[y][x] || i == forest.trees.len() - 1 {
                            scene_score *= count + 1;
                            break;
                        }
                    }
                    scene_score
                })
        })
        .max()
        .unwrap()
}

struct Forest {
    trees: Vec<Vec<i32>>,
    visibility: Vec<Vec<i32>>,
}

impl Forest {
    fn new(trees: Vec<Vec<i32>>) -> Self {
        let visibility = vec![vec![0; trees[0].len()]; trees.len()];
        Self { trees, visibility }
    }
    fn calculate_vis(&mut self) {
        // Visibility from left
        for (y, row) in self.trees.iter().enumerate() {
            let row_max = row.iter().max().unwrap();
            let mut max = -1;
            for (x, tree) in row.iter().enumerate() {
                if *tree > max {
                    max = *tree;
                    self.visibility[y][x] += 1;
                }

                if max == *row_max {
                    break;
                }
            }
        }
        // Visibility from right
        for (y, row) in self.trees.iter().enumerate() {
            let row_max = row.iter().max().unwrap();
            let mut max = -1;
            for (x, tree) in row.iter().enumerate().rev() {
                if *tree > max {
                    max = *tree;
                    self.visibility[y][x] += 1;
                }

                if max == *row_max {
                    break;
                }
            }
        }

        // Visibility from top
        for x in 0..self.trees[0].len() {
            let col_max = self.trees.iter().map(|row| row[x]).max().unwrap();
            let mut max = -1;
            for y in 0..self.trees.len() {
                if self.trees[y][x] > max {
                    max = self.trees[y][x];
                    self.visibility[y][x] += 1;
                }

                if max == col_max {
                    break;
                }
            }
        }

        // Visibility from bottom
        for x in 0..self.trees[0].len() {
            let mut max = -1;
            let col_max = self.trees.iter().map(|row| row[x]).max().unwrap();
            for y in (0..self.trees.len()).rev() {
                if self.trees[y][x] > max {
                    max = self.trees[y][x];
                    self.visibility[y][x] += 1;
                }

                if max == col_max {
                    break;
                }
            }
        }
    }

    fn _print_vis(&self) {
        for row in self.visibility.iter() {
            for vis in row.iter() {
                print!("{}", vis);
            }
            println!();
        }
    }
}

fn parse(input: &str) -> Forest {
    let v = input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as i32).collect())
        .collect();
    Forest::new(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_8_test.in");
        let mut forest = parse(&input);
        forest.calculate_vis();
        let p1 = part1(&forest);
        assert_eq!(p1, 21);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_8_test.in");
        let mut forest = parse(&input);
        forest.calculate_vis();
        let p2 = part2(&forest);
        assert_eq!(p2, 8);
    }
}

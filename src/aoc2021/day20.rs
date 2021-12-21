const ZONE: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 20);
    let mut im = parse(&input);
    let p1 = part_core(&mut im, 2);
    println!("Part 1: {}", p1);
    let p2 = part_core(&mut im, 50 - 2);
    println!("Part 2: {}", p2);
}

fn part_core(im: &mut Image, n: usize) -> usize {
    for i in 0..n {
        im.enhance(i % 2 == 1);
    }
    im.count()
}

struct Image {
    image: Vec<Vec<bool>>,
    flip: bool,
    algo: Vec<bool>,
}

fn parse(input: &str) -> Image {
    let mut parts = input.split("\n\n");
    let algo = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    let image = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<_>>>();
    let flip = algo[0];
    Image { image, flip, algo }
}

impl Image {
    fn enhance(&mut self, odd: bool) {
        let y_len = self.image.len();
        let x_len = self.image[0].len();
        let mut new_image = vec![vec![false; self.image[0].len() + 2]; self.image.len() + 2];
        for (y,row) in new_image.iter_mut().enumerate() {
            for (x, value) in row.iter_mut().enumerate() {
                let index = ZONE
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, (dx, dy))| {
                        let new_x = x as isize + dx - 1;
                        let new_y = y as isize + dy - 1;
                        if new_x < 0
                            || new_y < 0
                            || new_x >= x_len as isize
                            || new_y >= y_len as isize
                        {
                            if odd && self.flip {
                                1 << i
                            } else {
                                0
                            }
                        } else {
                            (self.image[new_y as usize][new_x as usize] as usize) << i
                        }
                    })
                    .sum::<usize>();
                *value = self.algo[index];
            }
        }
        self.image = new_image;
    }

    fn count(&self) -> usize {
        self.image
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c)
            .count()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_20_test.in");
        let mut im = parse(&input);
        let p1 = part_core(&mut im, 2);
        assert_eq!(p1, 35);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_20_test.in");
        let mut im = parse(&input);
        let p1 = part_core(&mut im, 50);
        assert_eq!(p1, 3351);
    }
}

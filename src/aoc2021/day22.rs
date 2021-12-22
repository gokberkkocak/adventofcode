pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 22);
    let v = parse(&input);
    let p1 = part1(&v);
    println!("Part 1: {}", p1);
    let p2 = part2(&v);
    println!("Part 2: {}", p2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl From<&str> for Cuboid {
    fn from(s: &str) -> Self {
        let c = s
            .split(',')
            .flat_map(|p| {
                p.split('=')
                    .nth(1)
                    .unwrap()
                    .split("..")
                    .map(|v| v.parse::<i32>().unwrap())
            })
            .collect::<Vec<_>>();
        Cuboid::new(c[0], c[1], c[2], c[3], c[4], c[5])
    }
}

impl Cuboid {
    pub fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Self {
        Cuboid {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    pub fn intersect_split(&mut self, other: &Cuboid) -> Vec<Cuboid> {
        let mut result_vec = Vec::new();
        if (self.x_min <= other.x_max && self.x_max >= other.x_min)
            && (self.y_min <= other.y_max && self.y_max >= other.y_min)
            && (self.z_min <= other.z_max && self.z_max >= other.z_min)
        {
            // on x
            if self.x_min < other.x_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    other.x_min - 1,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.x_min = other.x_min;
            }
            if self.x_max > other.x_max {
                result_vec.push(Cuboid::new(
                    other.x_max + 1,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.x_max = other.x_max;
            }
            // on y
            if self.y_min < other.y_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    other.y_min - 1,
                    self.z_min,
                    self.z_max,
                ));
                self.y_min = other.y_min;
            }
            if self.y_max > other.y_max {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    other.y_max + 1,
                    self.y_max,
                    self.z_min,
                    self.z_max,
                ));
                self.y_max = other.y_max;
            }
            // on z
            if self.z_min < other.z_min {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    self.z_min,
                    other.z_min - 1,
                ));
                self.z_min = other.z_min;
            }
            if self.z_max > other.z_max {
                result_vec.push(Cuboid::new(
                    self.x_min,
                    self.x_max,
                    self.y_min,
                    self.y_max,
                    other.z_max + 1,
                    self.z_max,
                ));
                self.z_max = other.z_max;
            }
        } else {
            result_vec.push(*self)
        }
        result_vec
    }

    pub fn area(&self) -> usize {
        let mut result = 1;
        result *= (self.x_max - self.x_min) as usize + 1;
        result *= (self.y_max - self.y_min) as usize + 1;
        result *= (self.z_max - self.z_min) as usize + 1;
        result
    }
}

fn parse(input: &str) -> Vec<Cuboid> {
    input.lines().fold(Vec::<Cuboid>::new(), |mut acc, line| {
        let (switch_str, coords) = line.split_once(' ').unwrap();
        let switch = switch_str == "on";
        let mut cuboids = Vec::with_capacity(acc.len() + 24);
        let parsed_cuboid = Cuboid::from(coords);
        for oc in acc.iter_mut() {
            cuboids.append(&mut oc.intersect_split(&parsed_cuboid));
        }
        if switch {
            cuboids.push(parsed_cuboid);
        }
        cuboids
    })
}

fn part1(v: &[Cuboid]) -> usize {
    v.iter()
        .filter(|c| {
            c.x_min >= -50
                && c.x_max <= 50
                && c.y_min >= -50
                && c.y_max <= 50
                && c.z_min >= -50
                && c.z_max <= 50
        })
        .map(|c| c.area())
        .sum()
}

fn part2(v: &[Cuboid]) -> usize {
    v.iter().map(|c| c.area()).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_1() {
        let input = "on x=10..12,y=10..12,z=10..12\n\
                            on x=11..13,y=11..13,z=11..13\n\
                            off x=9..11,y=9..11,z=9..11\n\
                            on x=10..10,y=10..10,z=10..10";
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 39);
    }

    #[test]
    fn test_1_2() {
        let input = crate::util::read_file("inputs/2021_22_test_1.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 590784);
    }

    #[test]
    fn test_1_3() {
        let input = crate::util::read_file("inputs/2021_22_test_2.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 474140);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_22_test_2.in");
        let v = parse(&input);
        let p2 = part2(&v);
        assert_eq!(p2, 2758514936282235);
    }
}

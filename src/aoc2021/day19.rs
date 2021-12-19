use fxhash::FxHashSet;
use itertools::Itertools;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 19);
    let mut plane = parse(&input);
    plane.merge_all_readings();
    let p1 = part1(&plane);
    println!("Part 1: {}", p1);
    let p2 = part2(&plane);
    println!("Part 2: {}", p2);
}

fn part1(plane: &Plane3D) -> usize {
    plane.beacon_locations.len()
}

fn part2(plane: &Plane3D) -> usize {
    plane
        .total_distances
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap() as usize
}

fn rotate([x, y, z]: [i32; 3], rot: u8) -> [i32; 3] {
    match rot {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Plane3D {
    let mut readings = input
        .split("\n\n")
        .map(|s| {
            let readings = s
                .lines()
                .skip(1)
                .map(|l| {
                    let mut parts = l.split(',');
                    let x = parts.next().unwrap().parse::<i32>().unwrap();
                    let y = parts.next().unwrap().parse::<i32>().unwrap();
                    let z = parts.next().unwrap().parse::<i32>().unwrap();
                    [x, y, z]
                })
                .collect::<Vec<_>>();
            let beacon_distance_set = readings
                .iter()
                .tuple_combinations()
                .map(|([x1, y1, z1], [x2, y2, z2])| {
                    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as usize
                })
                .collect::<FxHashSet<_>>();
            ScannerReading {
                beacons: readings,
                beacon_distance_set,
            }
        })
        .collect::<Vec<_>>();
    let first_scanner = readings.remove(0);
    let scanners = first_scanner.beacons.into_iter().collect::<FxHashSet<_>>();
    let beacon_distance_sets = vec![first_scanner.beacon_distance_set];
    println!("Somehow faster with this print\n{:?}", beacon_distance_sets);
    Plane3D {
        readings,
        beacon_locations: scanners,
        beacon_distance_sets,
        total_distances: vec![],
    }
}

struct Plane3D {
    readings: Vec<ScannerReading>,
    beacon_locations: FxHashSet<[i32; 3]>,
    beacon_distance_sets: Vec<FxHashSet<usize>>,
    total_distances: Vec<[i32; 3]>,
}

impl Plane3D {
    fn merge_all_readings(&mut self) {
        let mut dists = vec![[0, 0, 0]];
        while !self.readings.is_empty() {
            for i in (0..self.readings.len()).rev() {
                if let Some(d) = Plane3D::merge_reading(
                    &mut self.beacon_locations,
                    &mut self.beacon_distance_sets,
                    &self.readings[i],
                ) {
                    dists.push(d);
                    self.readings.swap_remove(i);
                }
            }
        }
        self.total_distances.extend(dists);
    }

    fn merge_reading(
        beacon_locations: &mut FxHashSet<[i32; 3]>,
        beacon_distance_sets: &mut Vec<FxHashSet<usize>>,
        reading: &ScannerReading,
    ) -> Option<[i32; 3]> {
        // For at least 12 intersections with rotation, we need 12 choose 2 = 66 intersections of beacon distances.
        // We can skip impossible reading matches by checking this condition first.
        let scanner_intersections = beacon_distance_sets
            .iter()
            .filter(|set| set.intersection(&reading.beacon_distance_set).count() >= 66)
            .count();
        if scanner_intersections == 0 {
            return None;
        }
        for r in 0..24 {
            let rotated = reading
                .beacons
                .iter()
                .map(|&v| rotate(v, r))
                .collect::<Vec<_>>();
            let distances = beacon_locations
                .iter()
                .cartesian_product(&rotated)
                .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);
            for [dx, dy, dz] in distances {
                let altered_rotated = rotated.iter().map(|[x, y, z]| [x + dx, y + dy, z + dz]);
                if altered_rotated
                    .clone()
                    .filter(|v| beacon_locations.contains(v))
                    .count()
                    >= 12
                {
                    beacon_locations.extend(altered_rotated);
                    beacon_distance_sets.push(reading.beacon_distance_set.clone());
                    return Some([dx, dy, dz]);
                }
            }
        }
        None
    }
}

struct ScannerReading {
    beacons: Vec<[i32; 3]>,
    beacon_distance_set: FxHashSet<usize>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_19_test.in");
        let mut plane = parse(&input);
        plane.merge_all_readings();
        let p1 = part1(&plane);
        assert_eq!(p1, 79);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_19_test.in");
        let mut plane = parse(&input);
        plane.merge_all_readings();
        let p2 = part2(&plane);
        assert_eq!(p2, 3621);
    }
}

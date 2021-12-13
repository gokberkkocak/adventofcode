use crate::util::get_puzzle_input;

pub fn run() {
    let input: usize = get_puzzle_input(2018, 11).parse().unwrap();
    part1(input);
    part2_summed(input);
}

fn default_fuel_matrix(input: usize) -> Vec<Vec<isize>> {
    let mut fuel_matrix = vec![vec![0isize; 300]; 300];
    for (i, inner) in fuel_matrix.iter_mut().enumerate() {
        for (j, v) in inner.iter_mut().enumerate() {
            let rack_id = i + 10;
            let power = (((((rack_id * j) + input) * rack_id) / 100) % 10) as isize - 5;
            *v = power;
        }
    }
    fuel_matrix
}

/// Source : https://en.wikipedia.org/wiki/Summed-area_table#
fn summed_area_fuel_matrix(input: usize) -> Vec<Vec<isize>> {
    let mut fuel_matrix = vec![vec![0isize; 301]; 301];
    for i in 1..301 {
        for j in 1..301 {
            let rack_id = i + 10;
            let power = (((((rack_id * j) + input) * rack_id) / 100) % 10) as isize - 5;
            fuel_matrix[i][j] =
                power + fuel_matrix[i - 1][j] + fuel_matrix[i][j - 1] - fuel_matrix[i - 1][j - 1];
        }
    }
    fuel_matrix
}

fn part1(input: usize) {
    let fuel_matrix = default_fuel_matrix(input);
    let mut max_fuel = isize::MIN;
    let (mut x, mut y) = (None, None);
    for i in 0..=297 {
        for j in 0..=297 {
            let mut fuel = 0;
            for x in 0..3 {
                for y in 0..3 {
                    fuel += fuel_matrix[i + x][j + y];
                }
            }
            if fuel > max_fuel {
                max_fuel = fuel;
                x = Some(i);
                y = Some(j);
            }
        }
    }

    // println!("{:?}", fuel_matrix);
    if let Some(i) = x {
        if let Some(j) = y {
            println!("{},{}", i, j);
        }
    }
}

fn part2_summed(input: usize) {
    let fuel_matrix = summed_area_fuel_matrix(input);
    let mut max_fuel = isize::MIN;
    let (mut x, mut y) = (None, None);
    let mut size = 0;
    for s in 1..=300 {
        for i in s..=300 {
            for j in s..=300 {
                let fuel = fuel_matrix[i][j] - fuel_matrix[i - s][j] - fuel_matrix[i][j - s]
                    + fuel_matrix[i - s][j - s];
                if fuel > max_fuel {
                    max_fuel = fuel;
                    x = Some(i);
                    y = Some(j);
                    size = s;
                }
            }
        }
    }
    if let Some(i) = x {
        if let Some(j) = y {
            println!("{},{},{}", i, j, size);
        }
    }
}

#[allow(dead_code)]
fn part2_naive(input: usize) {
    let fuel_matrix = default_fuel_matrix(input);
    let mut max_fuel = isize::MIN;
    let (mut x, mut y) = (None, None);
    let mut size = 0;
    for i in 0..=298 {
        for j in 0..=298 {
            'm: for s in 2..299 {
                let mut fuel = 0;
                for x in 0..s {
                    for y in 0..s {
                        if i + x > 299 || j + y > 299 {
                            break 'm;
                        }
                        fuel += fuel_matrix[i + x][j + y];
                    }
                }
                if fuel > max_fuel {
                    max_fuel = fuel;
                    x = Some(i);
                    y = Some(j);
                    size = s;
                }
            }
        }
    }

    // println!("{:?}", fuel_matrix);
    if let Some(i) = x {
        if let Some(j) = y {
            println!("{},{},{}", i, j, size);
        }
    }
}

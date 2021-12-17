use std::collections::HashSet;

use crate::util::get_puzzle_input;

pub struct Point {
    value: u8,
    owners: Vec<String>,
}

pub fn run() {
    let input = get_puzzle_input(2018, 3);
    let mut matrix: Vec<Vec<Point>> = Vec::with_capacity(1000);

    //empty 1000x1000 matrix
    for _i in 0..1000 {
        let mut a: Vec<Point> = Vec::with_capacity(1000);
        for _j in 0..1000 {
            let p = Point {
                value: 0,
                owners: Vec::new(),
            };
            a.push(p);
        }
        matrix.push(a);
    }
    let mut ids = HashSet::new();
    let mut count = 0;
    for line in input.lines() {
        let s_x: usize;
        let s_y: usize;
        let x: usize;
        let y: usize;
        let mut l = line.split_whitespace();
        let id = l.next().unwrap();
        ids.insert(id.to_string());
        l.next();
        let mut skips = l.next().unwrap();
        skips = &skips[0..skips.len() - 1];
        let mut m = skips.split(',');
        s_x = m.next().unwrap().parse().unwrap();
        s_y = m.next().unwrap().parse().unwrap();
        let dims = l.next().unwrap();
        let mut k = dims.split('x');
        x = k.next().unwrap().parse().unwrap();
        y = k.next().unwrap().parse().unwrap();
        for i in matrix.iter_mut().skip(s_x).take(x) {
            for j in i.iter_mut().skip(s_y).take(y) {
                if j.value == 0 {
                    j.value = 1;
                    j.owners.push(id.to_string());
                } else if j.value == 1 {
                    count += 1;
                    j.owners.push(id.to_string());
                    j.value = 2;
                } else {
                    j.owners.push(id.to_string());
                }
            }
        }
    }

    for i in matrix.iter_mut() {
        for j in i {
            if j.value > 1 {
                for o in j.owners.iter() {
                    if ids.contains(o) {
                        ids.remove(o);
                    }
                }
            }
        }
    }
    println!("overlap {}", count);
    println!("non-conflicted {:?}", ids);
}

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 17);
    let hb = parse(&input);
    let p1 = part1(&hb);
    println!("Part 1: {}", p1);
    let p2 = part2(&hb);
    println!("Part 1: {}", p2);
}

struct HitBox {
    x_max: i32,
    y_max: i32,
    x_min: i32,
    y_min: i32,
}

impl HitBox {
    fn new(x_max: i32, y_max: i32, x_min: i32, y_min: i32) -> HitBox {
        HitBox {
            x_max,
            y_max,
            x_min,
            y_min,
        }
    }
}

fn parse(input: &str) -> HitBox {
    let mut parts = input.strip_prefix("target area: ").unwrap().split(", ");
    let x_parts = parts
        .next()
        .unwrap()
        .strip_prefix("x=")
        .unwrap()
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let y_parts = parts
        .next()
        .unwrap()
        .strip_prefix("y=")
        .unwrap()
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    HitBox::new(x_parts[1], y_parts[1], x_parts[0], y_parts[0])
}

fn part1(hb: &HitBox) -> usize {
    // when y=0 we have either v_y_init or -vy_init speed.
    // on the way down, in next step we can have -v_y_init-1 speed and at maximum we can hit the bottom of the box with this speed.
    // any more than that and we will miss the target.
    // v_x can get 0 eventually so before ay step before v_y_init+1 steps we can set x to correct zone and it will stay in it. 
    // so, y_min = -vy_init - 1
    let v_y_init_max = (-hb.y_min - 1).abs() as usize;
    // gauss sum for height
    v_y_init_max * (v_y_init_max + 1) / 2
}

fn part2(hb: &HitBox) -> usize {
    let v_y_init_max = (-hb.y_min - 1).abs(); // we already proved this in part1
    let v_y_init_min = hb.y_min; // if more speed we overshoot the target in one step
    let v_x_init_max = hb.x_max; // if more speed we overshoot the target in one step
    let v_x_init_min = 0; // we can't go left
    (v_y_init_min..=v_y_init_max)
        .flat_map(|v_y| {
            (v_x_init_min..=v_x_init_max).filter_map(move |v_x| try_velocity(hb, v_x, v_y))
        })
        .count()
}

fn try_velocity(hb: &HitBox, mut v_x: i32, mut v_y: i32) -> Option<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    loop {
        x += v_x;
        y += v_y;
        v_x -= v_x.signum();
        v_y -= 1;
        match (
            hb.x_min <= x && x <= hb.x_max,
            hb.y_min <= y && y <= hb.y_max,
        ) {
            (true, true) => return Some((v_x, v_y)),
            (false, _) if v_x == 0 => return None,
            (_, false) if v_y < 0 && y < hb.y_min => return None,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2021_17_test.in");
        let v = parse(&input);
        let p1 = part1(&v);
        assert_eq!(p1, 45);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2021_17_test.in");
        let v = parse(&input);
        let p2 = part2(&v);
        assert_eq!(p2, 112);
    }
}

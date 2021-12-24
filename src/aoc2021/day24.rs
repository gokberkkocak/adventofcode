pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 24);
    let (p1, p2) = core(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn core(input: &str) -> (usize, usize) {
    let mut v_max = vec![0; 14];
    let mut v_min = vec![0; 14];
    let mut stack = vec![];
    let it_ins = input
        .split("inp w\n")
        .skip(1)
        .map(|s| s.lines().collect::<Vec<_>>());
    for (now, v_ins) in it_ins.enumerate() {
        let z_division = v_ins[3]
            .strip_prefix("div z ")
            .unwrap()
            .parse::<i8>()
            .unwrap();
        if z_division == 1 {
            // x gets 1
            // z keep prev_w+ prev_v14
            let f = v_ins[14]
                .strip_prefix("add y ")
                .unwrap()
                .parse::<i8>()
                .unwrap();
            stack.push((now, f));
        } else if z_division == 26 {
            // z gets reset to 0
            // x gets prev_w + prev_v14 + now_v3. if x == now_w, x gets 0 which make z 0
            let (mut prev, x) = stack.pop().unwrap();
            let mut diff = x + v_ins[4]
                .strip_prefix("add x ")
                .unwrap()
                .parse::<i8>()
                .unwrap();
            let mut now = now;
            // sign of diff alters things
            if diff < 0 {
                core::mem::swap(&mut now, &mut prev);
                diff = -diff;
            }
            // set
            v_max[now] = 9;
            v_max[prev] = 9 - diff as u8;
            v_min[now] = 1 + diff as u8;
            v_min[prev] = 1;
        }
    }
    let p1 = v_max
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as usize * 10usize.pow(i as u32))
        .sum::<usize>();
    let p2 = v_min
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as usize * 10usize.pow(i as u32))
        .sum::<usize>();
    (p1, p2)
}

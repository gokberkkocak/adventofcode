pub trait AOCYear {
    fn new() -> Box<dyn AOCYear>
    where
        Self: Sized;
    fn day1(&self);
    fn day2(&self);
    fn day3(&self);
    fn day4(&self);
    fn day5(&self);
    fn day6(&self);
    fn day7(&self);
    fn day8(&self);
    fn day9(&self);
    fn day10(&self);
    fn day11(&self);
    fn day12(&self);
    fn day13(&self);
    fn day14(&self);
    fn day15(&self);
    fn day16(&self);
    fn day17(&self);
    fn day18(&self);
    fn day19(&self);
    fn day20(&self);
    fn day21(&self);
    fn day22(&self);
    fn day23(&self);
    fn day24(&self);
    fn day25(&self);
}

impl dyn AOCYear {
    pub fn run_all(&self) {
        self.day1();
        self.day2();
        self.day3();
        self.day4();
        self.day5();
        self.day6();
        self.day7();
        self.day8();
        self.day9();
        self.day10();
        self.day11();
        self.day12();
        self.day13();
        self.day14();
        self.day15();
        self.day16();
        self.day17();
        self.day18();
        self.day19();
        self.day20();
        self.day21();
        self.day22();
        self.day23();
        self.day24();
        self.day25();
    }

    pub fn run_day(&self, day: u32) {
        match day {
            1 => self.day1(),
            2 => self.day2(),
            3 => self.day3(),
            4 => self.day4(),
            5 => self.day5(),
            6 => self.day6(),
            7 => self.day7(),
            8 => self.day8(),
            9 => self.day9(),
            10 => self.day10(),
            11 => self.day11(),
            12 => self.day12(),
            13 => self.day13(),
            14 => self.day14(),
            15 => self.day15(),
            16 => self.day16(),
            17 => self.day17(),
            18 => self.day18(),
            19 => self.day19(),
            20 => self.day20(),
            21 => self.day21(),
            22 => self.day22(),
            23 => self.day23(),
            24 => self.day24(),
            25 => self.day25(),
            _ => unreachable!(),
        }
    }
}

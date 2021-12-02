use std::{collections::HashMap};

use chrono::{Datelike, Duration, NaiveDateTime, Timelike};

use crate::util::get_puzzle_input;

#[derive(Debug)]
struct Sleep {
    id: Option<u16>,
    _date: (i32, u32, u32),
    sleep_start: Vec<u8>,
    sleep_end: Vec<u8>,
    sleep_time: Vec<bool>,
}

impl Sleep {
    fn new(date: (i32, u32, u32)) -> Self {
        Self {
            id: None,
            _date: date,
            sleep_start: Vec::new(),
            sleep_end: Vec::new(),
            sleep_time: vec![false; 60],
        }
    }
    fn set_id(&mut self, id: u16) {
        self.id = Some(id);
    }
    fn add_start(&mut self, sleep_start: u8) {
        self.sleep_start.push(sleep_start);
    }
    fn add_end(&mut self, sleep_end: u8) {
        self.sleep_end.push(sleep_end);
    }
    fn calculate_sleep(&mut self) {
        assert_eq!(self.sleep_start.len(), self.sleep_end.len());
        let mut p = self.sleep_start.iter();
        let mut q = self.sleep_end.iter();
        while let Some(start) = p.next() {
            if let Some(end) = q.next() {
                for i in *start..*end {
                    self.sleep_time[i as usize] = true;
                }
            }
        }
    }
}
#[derive(Debug)]
struct GuardSleep {
    id: u16,
    total_sleep_time: u64,
    sleep_data: Vec<u16>,
}

impl GuardSleep {
    fn new(id: u16) -> Self {
        Self {
            id,
            total_sleep_time: 0,
            sleep_data: vec![0; 60],
        }
    }
    fn add_new_data(&mut self, given_sleep_data: &Vec<bool>) {
        assert_eq!(given_sleep_data.len(), self.sleep_data.len());
        for (i, flag) in given_sleep_data.iter().enumerate() {
            if *flag {
                self.sleep_data[i] += 1;
                self.total_sleep_time += 1;
            }
        }
    }
}

pub fn run() {
    let file_contents = get_puzzle_input(2018, 4);
    let mut sleep_data: HashMap<(i32, u32, u32), Sleep> = HashMap::new();
    for line in file_contents.lines() {
        let iter = &mut line[1..].split("] ");
        if let Some(date) = iter.next() {
            let date = format!("{}:00", date);
            let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
            let date = time_adjust(date);
            let day_id = get_day(&date);
            let day_data = sleep_data.entry(day_id).or_insert(Sleep::new(day_id));
            let minute = get_time(&date) as u8;
            if let Some(action) = iter.next() {
                if action.starts_with("falls") {
                    day_data.add_start(minute);
                } else if action.starts_with("wakes") {
                    day_data.add_end(minute);
                } else if action.starts_with("Guard") {
                    if let Some(id) = action[6..].split_whitespace().next() {
                        let id = &id[1..];
                        let id = id.parse::<u16>().unwrap();
                        day_data.set_id(id);
                    }
                }
            }
        }
    }

    let mut guard_sleep_data: HashMap<u16, GuardSleep> = HashMap::new();
    for (_, v) in &mut sleep_data {
        v.calculate_sleep();
        let guard = guard_sleep_data
            .entry(v.id.unwrap())
            .or_insert(GuardSleep::new(v.id.unwrap()));
        guard.add_new_data(&v.sleep_time);
    }

    for (_g, v) in &guard_sleep_data {
        println!("{} {} {:?}", v.id, v.total_sleep_time, v.sleep_data);
    }

    part_1(&guard_sleep_data);
    part_2(&guard_sleep_data);
}

fn time_adjust(date: NaiveDateTime) -> NaiveDateTime {
    if date.hour() == 23 {
        let new_date = date.date() + Duration::days(1);
        return new_date.and_hms(0, 0, 0);
    }
    return date;
}

fn get_day(date: &NaiveDateTime) -> (i32, u32, u32) {
    (date.date().year(), date.date().month(), date.date().day())
}

fn get_time(date: &NaiveDateTime) -> u32 {
    date.minute()
}

fn part_1(guard_sleep_data: &HashMap<u16, GuardSleep>) {
    let mut max_sleep = 0;
    let mut sleepy: (u16, u16) = (0, 0);
    for (id, v) in guard_sleep_data {
        if v.total_sleep_time > max_sleep {
            max_sleep = v.total_sleep_time;
            let mut most_sleepy = 0;
            let mut most_sleepy_at = 0;
            for (i, x) in v.sleep_data.iter().enumerate() {
                if *x > most_sleepy {
                    most_sleepy_at = i as u16;
                    most_sleepy = *x;
                }
            }
            sleepy = (*id, most_sleepy_at);
        }
    }
    dbg!(sleepy);
    let result1 = sleepy.0 as u64 * sleepy.1 as u64;
    println!("{}", result1);
}

fn part_2(guard_sleep_data: &HashMap<u16, GuardSleep>) {
    let mut max_sleepy_time = 0;
    let mut max_sleepy_id = 0;
    let mut max_sleepy_when = 0;
    for (id, v) in guard_sleep_data {
        let guard_max = v
            .sleep_data
            .iter()
            .enumerate()
            .max_by_key(|&(_i, v)| v)
            .unwrap();
        if max_sleepy_time < *guard_max.1 {
            max_sleepy_time = *guard_max.1;
            max_sleepy_id = *id;
            max_sleepy_when = guard_max.0;
        }
    }
    dbg!(max_sleepy_id, max_sleepy_when);
    let result2 = max_sleepy_when as u64 * max_sleepy_id as u64;
    println!("{}", result2);
}

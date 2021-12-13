use std::{collections::HashMap, collections::HashSet};

use crate::util::get_puzzle_input;

pub fn run() {
    let input = get_puzzle_input(2018, 7);
    let mut task_precedence = HashMap::new();
    for line in input.lines() {
        let line = line
            .bytes()
            .skip(1)
            .filter(|x| x.is_ascii_uppercase())
            .collect::<Vec<u8>>();
        assert_eq!(2, line.len());
        let pre = line[0];
        let task = line[1];
        task_precedence
            .entry(task)
            .or_insert(HashSet::new())
            .insert(pre);
        let _ = task_precedence.entry(pre).or_insert(HashSet::new()); //also push for pre task as well if a task has never have pre
    }
    // println!("{:?}", task_precedence);
    part_2(&mut task_precedence);
}

#[allow(dead_code)]
fn part_1(task_precedence: &mut HashMap<u8, HashSet<u8>>) {
    let mut result = vec![];
    let n = task_precedence.len();
    while result.len() != n {
        let next = *task_precedence
            .iter()
            .filter(|&(_, pre)| pre.len() == 0)
            .min_by_key(|&(task, _)| *task)
            .unwrap()
            .0;
        result.push(next);
        task_precedence.remove(&next);
        task_precedence.iter_mut().for_each(|(_, pre)| {
            pre.remove(&next);
        });
    }
    let result = String::from_utf8(result).unwrap();
    println!("{}", result);
}

fn part_2(task_precedence: &mut HashMap<u8, HashSet<u8>>) {
    let mut result = vec![];
    let n = task_precedence.len();
    let nb_total_workers = 5;
    let mut workers = vec![vec![]; nb_total_workers];
    let mut task_finish = HashMap::new();
    while result.len() != n {
        while let Some(next_task) = task_precedence
            .iter()
            .filter(|&(_, pre)| pre.len() == 0)
            .map(|(&t, _)| t)
            .next()
        {
            result.push(next_task);
            task_precedence.remove(&next_task);
            let next_worker = workers
                .iter()
                .enumerate()
                .min_by_key(|&(_, x)| x.len())
                .unwrap()
                .0;
            (0..get_len(next_task)).for_each(|_| workers[next_worker].push(next_task));
            task_finish
                .entry(next_task)
                .or_insert(workers[next_worker].len());

            if task_finish.get(&next_task).unwrap()
                < &workers
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, a)| a.len())
                    .unwrap()
                    .0
            {
                task_precedence.iter_mut().for_each(|(_, pre)| {
                    pre.remove(&next_task);
                });
                task_finish.remove(&next_task);
            }
        }
        // println!("task finish map {:?}", task_finish);

        // sync workers
        let (task, min_finish) = task_finish.iter().min_by_key(|(_, &v)| v).unwrap();
        // println!("task {} finishes {}", task, min_finish);
        for w in workers.iter_mut() {
            while w.len() < *min_finish {
                w.push(0);
            }
        }
        task_precedence.iter_mut().for_each(|(_, pre)| {
            pre.remove(&task);
        });
        let task = *task;
        task_finish.remove(&task);
        // println!("precedence {:?}", task_precedence);
        // println!("workers {:?}", workers);
    }
    println!("{:?}", workers[0].len());
}

fn get_len(task: u8) -> u8 {
    task + 61 - "A".as_bytes()[0]
}

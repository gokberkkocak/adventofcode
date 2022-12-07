use std::collections::HashMap;

pub fn run() {
    let input = crate::util::get_puzzle_input(2022, 7);
    let lines = to_line_vec(&input);
    let (root, _) = traverse(&lines, 0);
    let p1 = part1(&root);
    println!("Part 1: {}", p1);
    let p2 = part2(&root);
    println!("Part 2: {}", p2);
}

pub fn part1(root: &Node) -> u64 {
    let sizes = all_subdir_sizes(&root);
    sizes.iter().filter(|size| **size < 100_000).sum()
}

pub fn part2(root: &Node) -> u64 {
    let sizes = all_subdir_sizes(&root);
    let root_size = root.size();
    *sizes
        .iter()
        .filter(|size| root_size - **size < 40_000_000) // root + update(30M) - deletion < 70M
        .min()
        .unwrap()
}

fn to_line_vec(input: &str) -> Vec<&str> {
    input.lines().map(|line| line).collect()
}

fn traverse(input: &[&str], mut line_number: usize) -> (Node, usize) {
    let mut content = HashMap::new();
    let mut total_size = 0;
    while line_number < input.len() {
        let line = input[line_number];
        match line.as_bytes()[0] {
            b'$' => {
                if line.as_bytes()[2] == b'c' {
                    // cd
                    let name = line.split_ascii_whitespace().skip(2).next().unwrap();
                    if name == ".." {
                        // return to parent
                        return (Node::Dir(content, total_size), line_number);
                    } else {
                        // enter directory
                        let (node, new_line_number) = traverse(input, line_number + 1);
                        line_number = new_line_number;
                        total_size += node.size();
                        content.insert(name.to_string(), node);
                    }
                } else {
                    // ls - ignore
                }
            }
            _ => {
                if line.as_bytes()[0] == b'd' {
                    // dir - ignore
                } else {
                    // file
                    let mut it = line.split_ascii_whitespace();
                    let size = it.next().unwrap().parse::<u64>().unwrap();
                    let name = it.next().unwrap();
                    content.insert(name.to_string(), Node::File(size));
                    total_size += size;
                }
            }
        }
        line_number += 1;
    }
    // return current since no "cd .."
    (Node::Dir(content, total_size), line_number)
}

impl Node {
    fn size(&self) -> u64 {
        match self {
            Node::Dir(_, total_size) => *total_size,
            Node::File(size) => *size,
        }
    }
}

fn all_subdir_sizes(node: &Node) -> Vec<u64> {
    match node {
        Node::Dir(content, total_size) => {
            let mut sizes = vec![*total_size];
            for child in content.values() {
                sizes.extend(all_subdir_sizes(child));
            }
            sizes
        }
        Node::File(_) => vec![],
    }
}

#[derive(Debug)]
pub enum Node {
    Dir(HashMap<String, Node>, u64),
    File(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2022_7_test.in");
        let lines = to_line_vec(&input);
        let (root, _) = traverse(&lines, 0);
        let p1 = part1(&root);
        assert_eq!(p1, 95437);
    }

    #[test]
    fn test_2() {
        let input = crate::util::read_file("inputs/2022_7_test.in");
        let lines = to_line_vec(&input);
        let (root, _) = traverse(&lines, 0);
        let p1 = part2(&root);
        assert_eq!(p1, 24933642);
    }
}

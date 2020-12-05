use std::collections::HashMap;

use crate::util::{get_puzzle_input, read_file};

#[derive(Debug, Clone)]
struct Node {
    nb_children: usize,
    nb_metadata: usize,
    children: Option<Vec<Node>>,
    metadata: Option<Vec<usize>>,
}

impl Node {
    fn new() -> Self {
        Self {
            nb_children: 0,
            nb_metadata: 0,
            children: None,
            metadata: None,
        }
    }
    fn set_nb_children(&mut self, nb_children: usize) {
        self.nb_children = nb_children;
        if nb_children > 0 {
            self.children = Some(vec![Node::new(); nb_children]);
        }
    }
    fn set_nb_metadata(&mut self, nb_metadata: usize) {
        self.nb_metadata = nb_metadata;
        if nb_metadata > 0 {
            self.metadata = Some(vec![0; nb_metadata]);
        }
    }
}

enum InputType {
    NbChildren,
    NbMetadataThenChildren,
    ReadMetadata,
}

pub fn run() {
    let input = get_puzzle_input(2018, 8);
    // let input = read_file("toy.txt");
    let mut it = input.split_ascii_whitespace();
    let mut init_node = Node::new();
    let current_node = &mut init_node;
    process_node(current_node, &mut it);
    let part1_sum = count_metadata(current_node);
    println!("p1 {}", part1_sum);
    let part2_sum = count_ref_metadata(current_node);
    println!("p2 {}", part2_sum);
}

fn process_node<'a>(node: &mut Node, it: &mut impl Iterator<Item = &'a str>) {
    let mut input_type = InputType::NbChildren;
    let mut which_metadata = 0;
    while let Some(v) = it.next() {
        let v = v.parse::<usize>().unwrap();
        match input_type {
            InputType::NbChildren => {
                node.set_nb_children(v);
                input_type = InputType::NbMetadataThenChildren;
            }
            InputType::NbMetadataThenChildren => {
                node.set_nb_metadata(v);
                if let Some(children) = &mut node.children {
                    for i in children.iter_mut() {
                        process_node(i, it);
                    }
                }
                input_type = InputType::ReadMetadata;
            }
            InputType::ReadMetadata => {
                if let Some(metadata) = &mut node.metadata {
                    metadata[which_metadata] = v;
                    which_metadata += 1;
                    if which_metadata == metadata.len() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn count_metadata(node: &Node) -> usize {
    let mut sum = 0;
    if let Some(metadata) = &node.metadata {
        sum = metadata.iter().sum();
    }
    if let Some(children) = &node.children {
        for i in children.iter() {
            sum += count_metadata(i);
        }
    }
    sum
}

fn count_ref_metadata(node: &Node) -> usize {
    let mut sum = 0;
    match &node.children {
        Some(children) => {
            let mut s = HashMap::new();
            if let Some(metadata) = &node.metadata {
                for &m_id in metadata.iter() {
                    if m_id - 1 < children.len() {
                        sum += *s
                            .entry(m_id)
                            .or_insert(count_ref_metadata(&children[m_id - 1]));
                    }
                }
            }
        }
        None => {
            if let Some(metadata) = &node.metadata {
                sum = metadata.iter().sum();
            }
        }
    }
    sum
}

use fxhash::{FxHashMap, FxHasher};
use std::hash::{Hash, Hasher};

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 12);
    let graph = parse(&input);
    let p1 = graph.create_paths(true);
    println!("Part 1: {}", p1);
    let p2 = graph.create_paths(false);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> Graph {
    let mut edges = FxHashMap::default();
    input
        .lines()
        .map(|line| {
            let mut it = line.split('-');
            let n_1 = it.next().unwrap();
            let n_2 = it.next().unwrap();
            (n_1, n_2)
        })
        .for_each(|(n_1, n_2)| {
            edges
                .entry(Node::new(n_1))
                .or_insert_with(Vec::new)
                .push(Node::new(n_2));
            edges
                .entry(Node::new(n_2))
                .or_insert_with(Vec::new)
                .push(Node::new(n_1));
        });
    Graph { edges }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Node {
    Start,
    End,
    Small(u64),
    Big(u64),
}

impl Node {
    fn new(s: &str) -> Self {
        if s == "start" {
            Node::Start
        } else if s == "end" {
            Node::End
        } else if s.chars().next().unwrap().is_lowercase() {
            let mut hs = FxHasher::default();
            s.hash(&mut hs);
            Node::Small(hs.finish())
        } else {
            let mut hs = FxHasher::default();
            s.hash(&mut hs);
            Node::Big(hs.finish())
        }
    }
}
struct Graph {
    edges: FxHashMap<Node, Vec<Node>>,
}

impl Graph {
    #[inline]
    fn get_connected_nodes(&self, node: &Node) -> impl Iterator<Item = Node> + '_ {
        self.edges.get(node).unwrap().iter().copied()
    }

    fn create_paths(&self, visited: bool) -> usize {
        let mut path = vec![];
        self.create_path(Node::Start, &mut path, visited)
    }

    fn create_path(&self, node: Node, path: &mut Vec<Node>, visited_small_twice: bool) -> usize {
        // add to path
        path.push(node);
        let nb_paths = self
            .get_connected_nodes(&node)
            .map(|n| match n {
                Node::Small(_) if (!visited_small_twice || !path.contains(&n)) => {
                    self.create_path(n, path, visited_small_twice || path.contains(&n))
                }
                Node::Big(_) => self.create_path(n, path, visited_small_twice),
                Node::End => 1,
                _ => 0,
            })
            .sum();
        // remove from path
        path.pop();
        // finished for this node
        nb_paths
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1_1() {
        let input = "start-A\n\
                        start-b\n\
                        A-c\n\
                        A-b\n\
                        b-d\n\
                        A-end\n\
                        b-end";
        let graph = parse(&input);
        assert_eq!(graph.create_paths(true), 10);
    }

    #[test]
    fn test_1_2() {
        let input = "dc-end\n\
                        HN-start\n\
                        start-kj\n\
                        dc-start\n\
                        dc-HN\n\
                        LN-dc\n\
                        HN-end\n\
                        kj-sa\n\
                        kj-HN\n\
                        kj-dc";
        let graph = parse(&input);
        assert_eq!(graph.create_paths(true), 19);
    }

    #[test]
    fn test_1_3() {
        let input = crate::util::read_file("inputs/2021_12_test.in");
        let graph = parse(&input);
        assert_eq!(graph.create_paths(true), 226);
    }

    #[test]
    fn test_2_1() {
        let input = "start-A\n\
                        start-b\n\
                        A-c\n\
                        A-b\n\
                        b-d\n\
                        A-end\n\
                        b-end";
        let graph = parse(&input);
        assert_eq!(graph.create_paths(false), 36);
    }

    #[test]
    fn test_2_2() {
        let input = "dc-end\n\
                        HN-start\n\
                        start-kj\n\
                        dc-start\n\
                        dc-HN\n\
                        LN-dc\n\
                        HN-end\n\
                        kj-sa\n\
                        kj-HN\n\
                        kj-dc";
        let graph = parse(&input);
        assert_eq!(graph.create_paths(false), 103);
    }

    #[test]
    fn test_2_3() {
        let input = crate::util::read_file("inputs/2021_12_test.in");
        let graph = parse(&input);
        assert_eq!(graph.create_paths(false), 3509);
    }
}

use std::collections::HashMap;

pub(crate) fn run() {
    let input = crate::util::get_puzzle_input(2021, 12);
    let graph = parse(&input);
    let p1 = graph.create_paths(true);
    println!("Part 1: {}", p1);
    let p2 = graph.create_paths(false);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> Graph {
    let mut edges = HashMap::new();
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
                .entry(n_1.to_string())
                .or_insert_with(Vec::new)
                .push(n_2.to_string());
            edges
                .entry(n_2.to_string())
                .or_insert_with(Vec::new)
                .push(n_1.to_string());
        });
    Graph { edges }
}

struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    #[inline]
    fn get_connected_nodes(&self, node: &str) -> impl Iterator<Item = &str> {
        self.edges.get(node).unwrap().iter().map(|n| n.as_str())
    }

    fn create_paths(&self, visited: bool) -> usize {
        let mut path = vec![];
        self.create_path("start", &mut path, visited)
    }

    fn create_path<'a, 'b>(
        &'a self,
        node: &'a str,
        path: &'b mut Vec<&'a str>,
        visited_small_twice: bool,
    ) -> usize {
        if node == "end" {
            return 1;
        }
        // add to path
        path.push(node);
        let nb_paths = self
            .get_connected_nodes(node)
            .map(|n| {
                if is_node_small(n) && (!visited_small_twice || !path.contains(&n)) {
                    self.create_path(n, path, visited_small_twice || path.contains(&n))
                } else if is_node_big(n) {
                    self.create_path(n, path, visited_small_twice)
                } else {
                    0
                }
            })
            .sum();
        // remove from path
        path.pop();
        // finished for this node
        nb_paths
    }
}
#[inline]
fn is_node_big(node: &str) -> bool {
    node.chars().next().unwrap().is_uppercase()
}
#[inline]
fn is_node_small(node: &str) -> bool {
    node.chars().next().unwrap().is_lowercase() && node != "start"
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

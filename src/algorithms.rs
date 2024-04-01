use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

// type BfsTree = HashMap<usize, (Option<usize>, usize)>;

pub struct BfsTree {
    pub tree: HashMap<usize, (Option<usize>, usize)>,
}

impl BfsTree {
    pub fn new() -> Self {
        BfsTree {
            tree: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: (Option<usize>, usize)) {
        self.tree.insert(key, value);
    }

    pub fn get(&self, key: &usize) -> Option<&(Option<usize>, usize)> {
        self.tree.get(key)
    }

    pub fn get_distance(&self, target: usize) -> usize {
        self.get(&target).unwrap().1
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        path.push(target);

        let mut current = self.get(&target).unwrap();
        while let Some(node) = current.0 {
            path.push(node);
            current = self.get(&node).unwrap();
        }
        path.reverse();
        path
    }
}

pub fn bfs(graph: &Graph, source: usize) -> BfsTree {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut bfs_tree: BfsTree = BfsTree::new();

    bfs_tree.insert(source, (None, 0));
    queue.push_back(source);
    visited.insert(source);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbours) = graph.adj_list.get(&current) {
            for &neighbour in neighbours {
                if !visited.contains(&neighbour) {
                    bfs_tree.insert(
                        neighbour,
                        (Some(current), bfs_tree.get(&current).unwrap().1 + 1),
                    );
                    visited.insert(neighbour);
                    queue.push_back(neighbour)
                }
            }
        }
    }

    bfs_tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new((1..6).collect());
        graph.add_edge(1, 2);
        graph.add_edge(1, 5);
        graph.add_edge(2, 5);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let bfs_tree: BfsTree = bfs(&graph, 1);
        let path: Vec<usize> = bfs_tree.get_path(3);
        assert_eq!(bfs_tree.get_distance(3), 2);
        assert_eq!(path, vec![1, 2, 3]);
    }
}

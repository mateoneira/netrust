use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(graph: Graph, source: usize) -> HashMap<usize, (Option<usize>, usize)> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut parent: HashMap<usize, (Option<usize>, usize)> = HashMap::new();

    parent.insert(source, (None, 0));
    queue.push_back(source);
    visited.insert(source);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbours) = graph.adj_list.get(&current) {
            for &neighbour in neighbours {
                if !visited.contains(&neighbour) {
                    parent.insert(
                        neighbour,
                        (Some(current), parent.get(&current).unwrap().1 + 1),
                    );
                    visited.insert(neighbour);
                    queue.push_back(neighbour)
                }
            }
        }
    }

    parent
}

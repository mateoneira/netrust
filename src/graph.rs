//! Basic graph module
//!
//! #Panics
//!
//! All methods will panic if given out-of-bounds element index.

use std::collections::{HashMap, HashSet};
///A compact graph representation using adjacency list
pub struct Graph {
    pub adj_list: HashMap<usize, Vec<usize>>,
}

/// Disjoint-set forest
/// A disjoint set representation using rooted trees
/// The disjoint set contains two vectors, one stores the roots
/// of the trees that represent each subset
/// the other stores the rank (size) of each tree
pub struct DisjointSet {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        //! Creates new disjoint set of size =n.s
        DisjointSet {
            parent: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n], // here we initialize the rank with a vector of size n and all elements equal to 0
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        //! find set representative
        //! use path-compression to make this faster
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }
    pub fn union(&mut self, x: usize, y: usize) {
        //! takes two elements and create a union of the sets they belong to
        //! The merging is done based on the rank of the roots. If the roots have different ranks, the root with the smaller rank is made a child of the root with the larger rank. If the roots have the same rank, one is arbitrarily made the child of the other, and the rank of the parent is incremented by one.
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root != y_root {
            if self.rank[x_root] < self.rank[y_root] {
                self.parent[x_root] = y_root;
            } else if self.rank[x_root] > self.rank[y_root] {
                self.parent[y_root] = x_root;
            } else {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }
}

impl Graph {
    pub fn new(vertices: HashSet<usize>) -> Self {
        //! Created new graph
        let mut adj_list = HashMap::new();
        for &vertex in &vertices {
            adj_list.insert(vertex, Vec::new());
        }
        Graph { adj_list }
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        //! Adds an undirected edge to the graph
        self.adj_list.get_mut(&src).unwrap().push(dest);
        self.adj_list.get_mut(&dest).unwrap().push(src)
    }

    pub fn get_nodes(&self) -> Vec<usize> {
        //! Returns a vector of all nodes in the graph
        self.adj_list.keys().cloned().collect()
    }

    pub fn num_v(&self) -> usize {
        self.adj_list.len()
    }

    pub fn num_e(&self) -> usize {
        self.adj_list
            .values()
            .map(|neighbours| neighbours.len())
            .sum::<usize>()
            / 2
    }

    pub fn from_edge_list(adj_list: Vec<(usize, usize)>) -> Self {
        //! Creates a graph from an adjacency list
        let mut nodes = HashSet::new();
        for &(src, dest) in &adj_list {
            nodes.insert(src);
            nodes.insert(dest);
        }

        let mut graph = Graph::new(nodes);
        for &(src, dest) in &adj_list {
            graph.add_edge(src, dest);
        }
        graph
    }

    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut disjoint_set = DisjointSet::new(self.num_v());
        for (&node, neighbours) in &self.adj_list {
            for &neighbour in neighbours {
                disjoint_set.union(node, neighbour);
            }
        }
        //creates a vector of vectors
        let mut components = vec![Vec::new(); self.num_v()];
        for node in self.get_nodes() {
            let parent = disjoint_set.find(node);
            components[parent].push(node);
        }
        components.into_iter().filter(|c| !c.is_empty()).collect()
    }

    pub fn print_adj_list(&self) {
        for (vertex, edges) in &self.adj_list {
            println!("Vertex {} -> {:?}", vertex, edges)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected_components() {
        let nodes: HashSet<usize> = [0, 1, 2, 3, 4].iter().cloned().collect();
        let mut graph = Graph::new(nodes);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        let components = graph.connected_components();
        assert_eq!(components.len(), 2);
        let c_1 = components[0].clone();
        let c_2 = components[1].clone();

        assert!(c_1.contains(&0));
        assert!(c_1.contains(&1));
        assert!(c_1.contains(&2));

        assert!(c_2.contains(&3));
        assert!(c_2.contains(&4));
    }

    #[test]
    fn test_find() {
        let mut ds: DisjointSet = DisjointSet::new(5);
        assert_eq!(ds.find(0), 0);
        assert_eq!(ds.find(4), 4);
    }

    #[test]
    fn test_union() {
        let mut ds: DisjointSet = DisjointSet::new(5);
        ds.union(1, 0);
        assert_eq!(ds.find(0), ds.find(1));
    }

    #[test]
    fn test_union_and_find() {
        let mut ds = DisjointSet::new(5);
        ds.union(1, 0);
        ds.union(2, 3);
        ds.union(1, 2);
        assert_eq!(ds.find(0), ds.find(3));
    }

    #[test]
    fn test_graph() {
        let mut graph = Graph::new((1..6).collect());
        graph.add_edge(1, 2);
        graph.add_edge(1, 5);
        graph.add_edge(2, 5);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        assert_eq!(graph.num_v(), 5);
        assert_eq!(graph.num_e(), 7);

        let expected = vec![
            (1, vec![2, 5]),
            (2, vec![1, 5, 3, 4]),
            (3, vec![2, 4]),
            (4, vec![2, 3, 5]),
            (5, vec![1, 2, 4]),
        ]
        .into_iter()
        .collect::<HashMap<usize, Vec<usize>>>();

        assert_eq!(graph.adj_list, expected);
    }
}

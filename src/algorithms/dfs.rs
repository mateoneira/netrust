use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct DfsNode {
    pub parent: Option<usize>,
    pub discovered: usize,
    pub finished: usize,
}

pub struct DfsTree {
    pub tree: HashMap<usize, DfsNode>,
}

# netrust

[![Rust](https://github.com/mateoneira/netrust/actions/workflows/rust.yml/badge.svg)](https://github.com/mateoneira/netrust/actions/workflows/rust.yml)

netrust is a Rust Library crate for working with graphs. It provides a basic graph structure and a set of graph algorithms that operate on this structure.

The library implements the basic algorithms introduced in: 

> Cormen, Thomas H., et al.
>  _Introduction to algorithms_. MIT press, 2022.

## Graph representation
* adjacency list representation: memory requirement $O(V+E)$

## Elementary graph algorithms
* **BFS**: creates a bfs tree where nodes contain the parent of every node and the distance to the source 
* **DFS**: creates a dfs forest formed of predecesor subgraphs where nodes contain a discovered and finished time.
* Kruskal minimum spanning tree
* Dijkstra's single-source shortest path

## Getting Started

To use Netrust in your Rust project, add the following line to your `Cargo.toml` file:

use netrust::algorithms;
use netrust::graph::Graph;
use std::env;
use std::fs;

fn main() {
    //read .txt of adj list
    let args: Vec<String> = env::args().collect();
    let filepath = args[1].clone();
    let contents = fs::read_to_string(&filepath).unwrap();
    //parse contents into vec of tuples
    let adj_list = contents
        .lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();
            let src: usize = parts[0].parse().unwrap();
            let dest: usize = parts[1].parse().unwrap();
            (src, dest)
        })
        .collect();
    //create graph
    let graph = Graph::from_adj_list(adj_list);
    //output basic properties
    let num_nodes = graph.num_v();
    let num_edges = graph.num_e();
    println!("number of nodes: {num_nodes}");
    println!("number of links: {num_edges}");
    graph.print_adj_list();

    //bfs of graph with source 2
    let bfs = algorithms::bfs(graph, 1);
    println!("distances from 1-3 = {}", bfs.get(&3).unwrap().1);
}

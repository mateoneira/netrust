use netrust::graph::Graph;
use std::env;
use std::fs;

fn main() {
    //read .txt of adj list
    let args: Vec<String> = env::args().collect();
    let filepath = args[1].clone();
    let contents = fs::read_to_string(&filepath).unwrap();
    println!("{filepath}");
    println!("with contents:\n{contents}");
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
    graph.print_adj_list();
    //output basic properties
}

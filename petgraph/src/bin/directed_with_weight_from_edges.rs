use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {

    let edges = [(0,1,2), (1,2,4), (2,0,-1)];
    //let edges = &[(0,1,2), (1,2,4), (2,0,-1)]; //イテレート可能なため、これでもいい
    //let edges = vec![(0,1,2), (1,2,4), (2,0,-1)]; //イテレート可能なため、これでもいい

    let mut directed_graph = petgraph::Graph::<usize,i32,petgraph::Directed>::from_edges(edges);
    //let directed_graph = petgraph::Graph::<usize,i32>::from_edges(edges);
    
    directed_graph.node_indices().enumerate().for_each(|(n,nodeix)| {directed_graph[nodeix] = n*2;});

    //println!("{:?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));
    println!("{:?}", Dot::with_config(&directed_graph, &[]));

}

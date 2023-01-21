use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {

    //let edges = [(0,1,2), (1,2,4), (2,0,-1)];
    let edges = &[(0,1,2), (1,2,4), (2,0,-1)]; //イテレート可能なため、これでもいい
    //let edges = vec![(0,1,2), (1,2,4), (2,0,-1)]; //イテレート可能なため、これでもいい

    let directed_graph = petgraph::Graph::<(),i32,petgraph::Directed>::from_edges(edges);
    //let directed_graph = petgraph::Graph::<(),i32>::from_edges(edges);

    println!("{:?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));
    //println!("{:?}", Dot::with_config(&directed_graph, &[]));

}

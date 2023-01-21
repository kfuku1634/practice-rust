use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {

    let edges = [(0,1), (1,2), (2,0)];
    //let edges = &[(0,1), (1,2), (2,0)]; //イテレート可能なため、これでもいい
    //let edges = vec![(0,1), (1,2), (2,0)]; //イテレート可能なため、これでもいい

    let directed_graph = petgraph::Graph::<(),(),petgraph::Directed>::from_edges(edges);
    //let directed_graph = petgraph::Graph::<(),()>::from_edges(edges);

    println!("{:?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));

}

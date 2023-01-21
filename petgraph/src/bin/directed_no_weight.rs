use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {
    let mut directed_graph = petgraph::Graph::<(),(),petgraph::Directed>::new();
    //let mut directed_graph = petgraph::Graph::<(),()>::new();

    let a = directed_graph.add_node(());
    let b = directed_graph.add_node(());

    directed_graph.add_edge(a,b,());

    println!("{:?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));

}

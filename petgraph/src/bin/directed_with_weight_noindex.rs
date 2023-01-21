use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {
    let mut directed_graph = petgraph::Graph::<usize,i32,petgraph::Directed>::new();

    let a = directed_graph.add_node(4);
    let b = directed_graph.add_node(10);

    directed_graph.add_edge(a,b, -3);

    println!("{:?}", Dot::with_config(&directed_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));

}

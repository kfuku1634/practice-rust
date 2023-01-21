use petgraph::dot::{Dot,Config};
use petgraph;

fn main() {
    let mut undirected_graph = petgraph::Graph::<usize,i32,petgraph::Undirected>::new_undirected();

    let a = undirected_graph.add_node(4);
    let b = undirected_graph.add_node(10);

    undirected_graph.add_edge(a,b, -3);

    println!("{:?}", Dot::with_config(&undirected_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));

}


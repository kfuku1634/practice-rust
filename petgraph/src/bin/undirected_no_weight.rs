use petgraph::dot::{Dot, Config};
use petgraph;

fn main() {
    let mut undirected_graph = petgraph::Graph::<(),(),petgraph::Undirected>::new_undirected();

    let a = undirected_graph.add_node(());
    let b = undirected_graph.add_node(());

    undirected_graph.add_edge(a,b,());

    println!("{:?}", Dot::with_config(&undirected_graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));

}

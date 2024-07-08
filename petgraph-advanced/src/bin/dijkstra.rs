use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;
fn main() {
    let mut graph: Graph<(), usize, Directed> = Graph::new();
    let a = graph.add_node(()); // node with no weight
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());
    let f = graph.add_node(());
    let g = graph.add_node(());
    let h = graph.add_node(());
    // z will be in another connected component
    let z = graph.add_node(());

    graph.extend_with_edges(&[
        (a, b, 1),
        (b, c, 1),
        (c, d, 1),
        (d, a, 1),
        (e, f, 1),
        (b, e, 1),
        (f, g, 1),
        (g, h, 1),
        (h, e, 1),
    ]);
    // a ----> b ----> e ----> f
    // ^       |       ^       |
    // |       v       |       v
    // d <---- c       h <---- g

    let aa = |e| {graph.edge_weight(e).unwrap()};
    let res = dijkstra(&graph, b, None,  a);
    println!("{:?}", res);
}

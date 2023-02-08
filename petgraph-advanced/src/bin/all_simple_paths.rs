use petgraph::algo;
use petgraph::prelude::*;
use itertools::Itertools;

fn main() {
    let mut graph: Graph<String, (), Undirected> = Graph::new_undirected();
    let a = graph.add_node("a".to_string()); // node with no weight
    let b = graph.add_node("b".to_string());
    let c = graph.add_node("c".to_string());
    let d = graph.add_node("d".to_string());
    let e = graph.add_node("e".to_string());
    let f = graph.add_node("f".to_string());
    let g = graph.add_node("g".to_string());
    let h = graph.add_node("h".to_string());
    // z will be in another connected component
    let z = graph.add_node("z".to_string());

    graph.extend_with_edges(&[
        (a, b),
        (b, c),
        (c, d),
        (d, a),
        (e, f),
        (b, e),
        (f, g),
        (g, h),
        (h, e),
    ]);

// a ----- b ----- e ----- f
// |       |       |       |
// |       |       |       |
// d ----- c       h ----- g

let res: Vec<Vec<_>> = algo::all_simple_paths(&graph, c, g, 1, None).collect();

    for i in res {
        i.iter()
            .map(|x| graph.node_weight(*x).unwrap())
            .intersperse(&String::from("->"))
            .for_each(|x| print!("{x}"));
        println!("")
    }
}

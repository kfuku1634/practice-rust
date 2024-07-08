use petgraph::algo::bellman_ford;
use petgraph::prelude::*;
use petgraph::Graph;

fn main() {
    let mut g = Graph::new();
    let a = g.add_node(()); // node with no weight
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    let f = g.add_node(());
    g.extend_with_edges(&[
        (0, 1, 2.0),
        (0, 3, 4.0),
        (1, 2, 1.0),
        (1, 5, 7.0),
        (2, 4, 5.0),
        (4, 5, 1.0),
        (3, 4, 1.0),
    ]);

    // Graph represented with the weight of each edge
    //
    //     2       1
    // a ----- b ----- c
    // | 4     | 7     |
    // d       f       | 5
    // | 1     | 1     |
    // \------ e ------/

    let path = bellman_ford(&g, a);
    assert_eq!(
        path,
        Ok((
            vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            vec![None, Some(a), Some(b), Some(a), Some(d), Some(e)]
        ))
    );
}

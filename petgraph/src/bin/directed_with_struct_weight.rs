use petgraph::dot::Dot;
use petgraph;

#[derive(Debug)]
struct MyWeight {
    weight: usize,
    name: String
}

fn main() {
    let mut directed_graph = petgraph::Graph::<MyWeight,(),petgraph::Directed>::new();

    let my_weight_a = MyWeight{weight:4, name:"a".to_string()};
    let my_weight_b = MyWeight{weight:10, name:"b".to_string()};
    let a = directed_graph.add_node(my_weight_a);
    let b = directed_graph.add_node(my_weight_b);

    directed_graph.add_edge(a,b,());


    println!("{:?}", Dot::with_config(&directed_graph, &[]));

}

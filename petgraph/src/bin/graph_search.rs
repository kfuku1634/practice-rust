use petgraph::prelude::*;
use petgraph::dot::{Dot,Config};

fn main(){
    let es = [(0,1),(1,2),(1,3),(3,4),(0,5)];
    let graph = Graph::<(),(), Undirected>::from_edges(es);

//let mut dfs = Dfs::new(&graph,NodeIndex::new(0));
//let mut dfs_order = vec![];
//while let Some(node) = dfs.next(&graph) {
    //println!("{:b}", dfs.discovered);
    //for i in 0..dfs.discovered.len() {
        //if dfs.discovered[i] {
            //print!("{}",1);
        //}else {
            //print!("{}",0);
        //}
    //}
    //println!();
    //dfs_order.push(node.index());
//}

let mut bfs = Bfs::new(&graph,NodeIndex::new(0));
let mut bfs_order = vec![];
while let Some(node) = bfs.next(&graph) {
    for i in 0..bfs.discovered.len() {
        if bfs.discovered[i] {
            print!("{}",1);
        }else {
            print!("{}",0);
        }
    }
    println!();
    bfs_order.push(node.index());
}

    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));
    //println!("{:?}",dfs_order);
    //println!("{:?}",bfs_order);
}


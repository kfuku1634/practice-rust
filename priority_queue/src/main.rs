use std::collections::BTreeMap;

fn main() {
    let mut bt_map: BTreeMap<_, _> = BTreeMap::new();

    for i in [1, 2, 3, 4, 1] {
        bt_map.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }

    for i in bt_map.iter() {
        eprintln!("i = {:?}", i);
    }
}

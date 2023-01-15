use itertools::Itertools as _;
use num::Integer;

fn main() {
    let a = (0..=10).step_by(2);
    let b = (1..=10).step_by(2);

    // 交互にiter
    assert_eq!(a.clone().interleave(b).collect::<Vec<usize>>(), (0..=10).collect::<Vec<usize>>());

    let c = vec![1,3];
    assert_eq!(a.interleave_shortest(c).collect::<Vec<usize>>(), (0..=4).collect::<Vec<usize>>());

    let a = vec![0,1,2];
    assert_eq!(a.clone().into_iter().intersperse(5).collect::<Vec<_>>(), vec![0,5,1,5,2]);

    // iterを読み込んでiterを作成
    let p: Vec<usize> = a.clone().iter().batching(|it| {
                    match it.next() {
                        None => None,
                        Some(x) => Some(x*2),
                    }
                }).collect();

    assert_eq!( p, vec![ 0, 2, 4]);

    let a = vec![ 2, 4, 2, 5, 3, 8, 3, 3 ,1 ,1];
    for (key, group) in &a.into_iter().group_by(|e| e.is_even()){
        assert_eq!( group.sum::<usize>(), 8);
    }
    
    let a = vec![ 3,3,3, 2,6,1,4,4,1,9];
    for chunk in &a.into_iter().chunks(3){
        assert_eq!( chunk.sum::<usize>(), 9);
    }

}

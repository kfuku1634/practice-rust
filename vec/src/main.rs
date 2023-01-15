use num::Integer;
use superslice::*;

fn main() {
    let mut a = vec![1,2,3];
    let mut b = vec![4,5,6];
    a.append(& mut b);
    assert_eq!(a, vec![1,2,3,4,5,6]);

    let mut a = vec!['a','a','k','z','z','z','a'];
    a.dedup();
    assert_eq!(a, vec!['a', 'k', 'z', 'a']);

    let mut a = vec![ 'a', 'A', 'a', 'B','b'];
    a.dedup_by(|a,b| a.eq_ignore_ascii_case(b));
    assert_eq!(a, vec!['a','B']);

    let mut a = vec![ 11, 1, 100, 500, 12];
    a.dedup_by_key( |x| *x%10);
    assert_eq!(a, vec![11,100,12]);

    let mut a = vec![1,2,3,4,5];
    let b: Vec<_> = a.drain(3..).collect();
    assert_eq!( b, &[4,5]);
    assert_eq!( a, &[1,2,3]);

    let mut a = vec![1];
    let b = &[2,3,4];
    a.extend_from_slice(b);
    assert_eq!(a, [1,2,3,4]);

    let mut a = vec![1,2,3];
    a.insert(1,4);
    assert_eq!(a, vec![1,4,2,3]);

    let mut a = vec![1];
    a.push(2);
    assert_eq!(a, vec![1,2]);
    assert_eq!(a.pop(), Some(2)); // from last.
    assert_eq!(a, vec![1]);

    let mut a = vec![1,2,3];
    assert_eq!( a.remove(1), 2);

    let mut a = vec![1,2,3];
    a.retain(|&x| x.is_odd());
    assert_eq!( a, vec![1,3]);

    let mut a = vec![1,2,3,4,5];
    let b = vec![8,9];
    let u = a.splice(3..=4, b).collect::<Vec<usize>>();
    assert_eq!(u, vec![4,5]);
    assert_eq!(a, vec![1,2,3,8,9]);

    let mut a = vec![1,2,3,4,5,6];
    let b = a.split_off(3);
    assert_eq!(a, vec![1,2,3]);
    assert_eq!(b, vec![4,5,6]);

    let mut a = vec![1,2,3,4,5,6];
    a.truncate(3);
    assert_eq!(a, vec![1,2,3]);

    let s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let idx = s.binary_search(&13).unwrap();
    assert_eq!(idx, 9);
    assert_eq!(idx, s.upper_bound(&13));
}

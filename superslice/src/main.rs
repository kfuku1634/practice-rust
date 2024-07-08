use superslice::Ext;

fn main() {

    let a = [10, 11, 13, 13, 15];

    // lower_bound: x以上を満たす最初のi
    // upper_bound: xより大きいを満たす最初のi
    assert_eq!(a.lower_bound(&9), 0);
    assert_eq!(a.upper_bound(&9), 0);

    assert_eq!(a.lower_bound(&10), 0);
    assert_eq!(a.upper_bound(&10), 1);

    assert_eq!(a.lower_bound(&11), 1);
    assert_eq!(a.upper_bound(&11), 2);

    assert_eq!(a.lower_bound(&12), 2);
    assert_eq!(a.upper_bound(&12), 2);

    assert_eq!(a.lower_bound(&13), 2);
    assert_eq!(a.upper_bound(&13), 4);

    assert_eq!(a.lower_bound(&14), 4);
    assert_eq!(a.upper_bound(&14), 4);

    assert_eq!(a.lower_bound(&15), 4);
    assert_eq!(a.upper_bound(&15), 5);

    assert_eq!(a.lower_bound(&16), 5);
    assert_eq!(a.upper_bound(&16), 5);

    // aを変換して、条件を適用できる
    assert_eq!(a.lower_bound_by_key( &22, |x| x*2), 1);
    assert_eq!(a.upper_bound_by_key( &22, |x| x*2), 2);

    assert_eq!(a.equal_range(&13), (2..4) );
    assert_eq!(a.equal_range_by_key(&26, |x| x*2), (2..4) );

    let mut a =  [1, 3, 3, 5];

    let x = a.next_permutation();
    assert_eq!( (x, a), (true, [1, 3, 5, 3]));

    let x = a.prev_permutation();
    assert_eq!( (x,a), (true, [1, 3, 3, 5]));

    let mut a = [ 'd', 'c', 'a', 'b'];
    let mut p = [  2,   3,   1,   0];
    a.apply_permutation(&mut p);
    assert_eq!(a, ['a', 'b', 'c', 'd']);

    let mut a = [ 'd', 'c', 'a', 'b'];
    let mut p = [  3,   2,   0 ,  1];
    a.apply_inverse_permutation(&mut p);
    assert_eq!(a, ['a', 'b', 'c', 'd']);

}

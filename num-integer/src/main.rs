// https://docs.rs/num-integer/0.1.42/num_integer/trait.Integer.html
use num::integer::{Integer,ExtendedGcd}; // bring in num_integer trait.
use num::traits::NumAssign;

fn main() {
    
    assert_eq!( ( 11).div_floor(& 4),  2); //  floor( 2.75) =  2
    assert_eq!( ( 11).div_floor(&-4), -3); //  floor(-2.75) = -3
    assert_eq!( (-11).div_floor(& 4), -3); //  floor(-2.75) = -3
    assert_eq!( (-11).div_floor(&-4),  2); //  floor( 2.75) =  2
    assert_eq!(   -11.div_floor(& 4), -2); // -floor( 2.75) = -2 // div_floor is prior to minus.

    assert_eq!(  11 %  4,  3 );
    assert_eq!(  11 % -4,  3 );
    assert_eq!( -11 %  4, -3 );
    assert_eq!( -11 % -4, -3 );

    assert_eq!( ( 11).mod_floor(& 4),  3);
    assert_eq!( ( 11).mod_floor(&-4), -1);
    assert_eq!( (-11).mod_floor(& 4),  1);
    assert_eq!( (-11).mod_floor(&-4), -3);

    let n = 11;
    let d = 4;
    assert!(n.div_floor(&d) * d + n.mod_floor(&d) == n); // mod_floor definition

    assert_eq!(9.gcd(&6), 3);

    assert_eq!(9.lcm(&6), 18);

    assert_eq!(9.is_multiple_of(&3), true);

    assert_eq!(4.is_even(), true);

    assert_eq!(3.is_odd(), true);

    assert_eq!( ( 11).div_rem(& 4), ( 2, 3));
    assert_eq!( ( 11).div_rem(&-4), (-2, 3));
    assert_eq!( (-11).div_rem(& 4), (-2,-3));
    assert_eq!( (-11).div_rem(&-4), ( 2,-3));

    // why div_rem and (div_floor,div_mod) is required method?
    // One is introduced to another.

    assert_eq!( ( 11).div_ceil(& 4),  3); //  ceil( 2.75) =  3
    assert_eq!( ( 11).div_ceil(&-4), -2); //  ceil(-2.75) = -2
    assert_eq!( (-11).div_ceil(& 4), -2); //  ceil(-2.75) = -2
    assert_eq!( (-11).div_ceil(&-4),  3); //  ceil( 2.75) =  3
    assert_eq!(   -11.div_ceil(& 4), -3); // -ceil( 2.75) = -3 // div_floor is prior to minus.

    assert_eq!((9).gcd_lcm(& 6), ( 3, 18) );

    // https://en.wikipedia.org/wiki/B%C3%A9zout%27s_identity
    // https://ja.wikipedia.org/wiki/%E3%83%99%E3%82%BA%E3%83%BC%E3%81%AE%E7%AD%89%E5%BC%8F
    fn check<A: Copy + Integer + NumAssign>(a: A, b: A) -> bool {
    let ExtendedGcd { gcd, x, y, .. } = a.extended_gcd(&b);
        gcd == x * a + y * b
    }
    assert!(check(10isize, 4isize));
    assert!(check(8isize,  9isize));


    assert_eq!( (11).div_mod_floor(&4), (2, 3));

    assert_eq!( (11).next_multiple_of(&4), 12);
    assert_eq!( (11).prev_multiple_of(&4),  8);
}

fn main() {
    let b_ptr;
    {
        let b = Vec::from([0, 1, 2]);
        b_ptr = b.as_ptr();
        unsafe {
            println!("address        : {:?}", b_ptr);
            println!("first element  : {:?}", *b_ptr);
            println!("second element : {:?}", *(b_ptr.add(1)));
            println!("thired element : {:?}", *(b_ptr.add(2)));
        }
    }
    unsafe {
        println!("address        : {:?}", b_ptr);
        println!("first element  : {:?}", *b_ptr);
        println!("second element : {:?}", *(b_ptr.add(1)));
        println!("thired element : {:?}", *(b_ptr.add(2)));
    }
}

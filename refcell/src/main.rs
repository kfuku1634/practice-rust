use std::cell::RefCell;

fn main() {
    println!("compiled successfully");

    //let mut x = 5;
    //let rx = &mut x;
    //let rx1 = &x; // not allowed 
    //println!("{}",rx1);
    //println!("{}",rx);


    let x = RefCell::new(5);
    let rx = x.borrow_mut();
    let rx1 = x.borrow(); 
    println!("{}",rx1);
    println!("{}",rx);

}

fn main() {
    let n:i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let l:i32 = rand::random(); 
    println!("{}",n+l);
}

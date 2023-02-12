fn main() {
    let n: i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}",n+1);
}

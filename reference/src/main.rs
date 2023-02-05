fn main() {
let x = 5;
println!("x   = {}",x);
println!("&x  = {:p}",&x);

let rx = &x;
println!("{}", rx);
println!("{:p}", rx);
println!("{:p}", &rx);

}

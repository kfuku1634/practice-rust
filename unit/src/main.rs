#[derive(Debug)]
struct Hoge;

struct Fuga {
    a: usize,
    b: (),
}

fn main() {
    hello();
    let a = Hoge;
    eprintln!("hoge = {:?}", a);
    let a = 1;
    if a % 2 == 0 {
        println!("hoge");
    }
    let a: () = ();

}

fn hello() {
    println!("hello");
}

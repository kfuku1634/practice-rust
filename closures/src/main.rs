fn main() {
    let b = 10;
    //let a = || println!("{}", b);
    //fn a() {
    //println!("{}", b); // <- // not allowed
    //}
    //a();
    //hoge(); // <- // not allowed

    let v = vec![1, 2, 3];
    let a = || {
        println!("{:?}", v);
    };
    a();
    println!("{:?}", v);
    a();

    let mut v = vec![1, 2, 3];
    let mut a = || {
        v.push(4);
        println!("{:?}", v);
    };
    a();
    v.push(5);
    //a(); // <- not allowed

    let mut v = vec![1, 2, 3];
    let mut a = move || {
        v.push(4);
        println!("{:?}", v);
    };
    a();
    //v.push(5); // <- not allowed
    //a(); // <- not allowed

struct Hoge {
    v: Vec<i32>,
}
impl Hoge {
    fn fuga(&self) {
        println!("{:?}", self.v)
    }
}
}

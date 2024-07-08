struct Hoge<T> {
    hoge_elm: T,
}

impl<T> Hoge<T> {
    fn ya(&self) {
        println!("ya");
    }
}

impl Hoge<i32> {
    fn hi(&self) {
        println!("hi");
    }
}


fn main() {
    let a = Hoge { hoge_elm: 10 };
    let b = Hoge { hoge_elm: "Hoge".to_string() };

    a.ya();
    a.hi();
    b.ya();
    //b.hi();
}

fn main() {
    let a = 10;
    println!("{}", a.is_positive());
    println!("{}", a.is_negative());

    let s = "fuga!";
    println!("{}", s.is_positive());
    println!("{}", s.is_negative());

    let v = vec![0,1];
    println!("{}", v.is_positive());
    println!("{}", v.is_negative());
}
trait MyMath {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}

impl<T: std::cmp::PartialOrd + Default> MyMath for T {
    fn is_positive(&self) -> bool {
        let a = T::default();
        a < *self
    }
    fn is_negative(&self) -> bool {
        let a = T::default();
        *self < a
    }
}

trait Hoge {
    fn p(&self) {}
}

impl<T: std::fmt::Display> Hoge for T {
    fn p(&self) {
        println!("{}", self.to_string());
    }
}


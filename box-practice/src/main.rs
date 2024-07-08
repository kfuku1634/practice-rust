trait Liquid {
    fn amount(&self) -> usize;
    fn double_amount(&self) -> usize {
        self.amount() * 2
    }
}

struct Water {
    amount: usize,
}

impl Liquid for Water {
    fn amount(&self) -> usize {
        self.amount
    }
}

trait Drink: Liquid {
    fn drink(&self) {
        println!("drink {}ml!", self.double_amount());
    }
}

struct Oil {
    amount: usize,
}

impl Liquid for Oil {
    fn amount(&self) -> usize {
        self.amount
    }
}

fn new_liquid(s: &str) -> Box<dyn Liquid> {
    if s == "water" {
        Box::new(Water { amount: 10 })
    } else {
        Box::new(Oil { amount: 10 })
    }
}

fn main() {
    //let a = Box::new(10);
    //let b = 10;
    //println!("{:p}", &a);
    //println!("{:p}", a);
    //println!("{:p}", &b);

    let some_liquid = new_liquid("water");
    println!("{:p}", &some_liquid);
    println!("{:p}", some_liquid);
}

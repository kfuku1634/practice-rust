trait Liquid {
    fn amount(&self) -> usize;
    fn double_amount(&self) -> usize {
        self.amount() * 2
    }
}

trait Drink: Liquid {
    fn drink(&self) {
        println!("drink {}ml!", self.double_amount());
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

impl Drink for Water {}

trait Pour {
    fn pour(&self) {
        println!("pour");
    }
}

impl Pour for Water {}
impl Pour for u8 {}

fn all_drink_for_you(d: &impl Drink) {
    println!("{}ml drink for you!", d.amount());
}

//fn all_drink_for_you<T:Drink>(d: &T) {
//println!("{}ml drink for you!", d.amount());
//}

//fn all_drink_for_you<T>(d: &T)
//where
//T: Drink,
//{
//println!("{}ml drink for you!", d.amount());
//}

fn some_liquid(var: &str) -> Box<dyn Liquid> {
    if var == "water" {
        Box::new(Water { amount: 10 })
    } else {
        Box::new(Oil { amount: 10 })
    }
}
fn main() {
    let liquid = some_liquid();
    println!("{}", liquid.amount());
}

struct Oil {
    amount: usize,
}

impl Liquid for Oil {
    fn amount(&self) -> usize {
        self.amount
    }
    fn double_amount(&self) -> usize {
        self.amount * 3
    }
}

trait Drinkable {
    fn drink(&self) {}
}

struct Coke();

impl Drinkable for Coke {
    fn drink(&self) {
        println!("drink Coke!");
    }
}
struct Tea();
impl Drinkable for Tea {
    fn drink(&self) {
        println!("drink Tea.");
    }
}

fn main() {
    let drinks: Vec<Box<dyn Drinkable>> = vec![Box::new(Coke()), Box::new(Tea())];
    for some_drink in drinks {
        some_drink.drink();
    }
}

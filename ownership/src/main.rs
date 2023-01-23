#[derive(Debug)]
struct Person {
    name: String,
    birth: i32,
}
fn main() {

    let my_name = "hoge".to_string();
    let birth_year = 1992;
    let s = Person {
        name: my_name,
        birth: birth_year,
    };

    //println!("{}", my_name);  // my_name variable is moved.
    println!("{}", birth_year); // birth_year variable can display because that is copied as primitive type.
    println!("{:?}", s);

    let mut v = vec![];
    v.push(s);
    //println!("{:?}",s); //s variable is moved to v;

    mutate_v(&mut v);

    println!("{:?}", v);
}

fn mutate_v( v: &mut Vec<Person> ) {
    v[0].birth = 2000;
}

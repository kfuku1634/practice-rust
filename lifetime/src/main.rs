struct Hoge<'a> {
    s: &'a str,
}

impl<'a> Hoge<'a> {
    fn first_str(&self, a: &'a str) -> &str {
        &a[0..1]
    }
}

fn main() {
    let a = "hogehoge".to_string();
    let mut c = "";
    {
        let b = "hoge".to_string();
        c = longest(&a, &b);
        //println!("{}", c);
    }

    let first_str = first_str("hoge");
    //println!("{}", first_str);

    let hoge = Hoge { s: "fuga" };
    let first_str = hoge.first_str("moge");
    //println!("{}", first_str);

    //let a = String::from("hoge");
    let mut d = "";
    {
        let a = "foo".to_string();
        d = &a;
    }
    println!("{}", d);

    let a = 10;
    let b = 20;

    let c = largest(&a, &b);
    //println!("{}", c);

    let an = ImportantExcerpt { part: "hoge" };
    an.announce_and_return_part("hoge");
}

fn first_str(a: &str) -> &str {
    &a[0..1]
}

fn largest<'a>(a: &'a usize, b: &'a usize) -> &'a usize {
    if a > b {
        a
    } else {
        b
    }
}

fn longest<'a>(lhd: &'a str, rhd: &'a str) -> &'a str {
    if lhd.len() > rhd.len() {
        lhd
    } else {
        rhd
    }
}

fn ret_string(some_str: &str) -> &str {
    some_str
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &'a str) -> &str {
        //println!("Attention please: {}", self.part);
        announcement
    }
}

struct Book<'a> {
    name: &'a str,
}

fn main() {
    let s = "cpp".to_string();

    let b = Book {
        name: s.as_str(),
    };

    println!("{}", b.name);
}

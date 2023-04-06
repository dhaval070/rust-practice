use std::collections::HashMap;

fn main() {
    let mut names = vec!["dhaval", "munna", "pop"];

    names.push("dd");
    println!("{:?}", names);

    names.remove(1);
    println!("{:?}", names);
    println!("{}", names.contains(&"pop"));

    let mut h = HashMap::new();
    h.insert("id", 2);

    println!("{:?}", h);

    match h.get("id") {
        Some(v) => println!("{:?}", v),
        None => println!("not found"),
    }

    if let h.get("id") {
        println!("if let got");
    }
}

use std::ops::Deref;

// automatic Deref coersion from &String to &str
fn display(s: &str) {
    println!("hello: {s}");
}

// Custom type Deref coersion
struct MyBox<T> (T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("going out of scope");
    }
}

fn main() {
    let str = String::from("bmw");

    display(&str);

    let n = 10;
    let mb = MyBox(n);

    assert_eq!(10, *mb);
}

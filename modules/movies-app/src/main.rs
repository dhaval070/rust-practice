extern crate movies_lib;

use movies_lib::add;

fn main() {
    let res = add(2, 5);

    println!("Hello, world! {}", res);
    movies_lib::movies::play("hii");
}

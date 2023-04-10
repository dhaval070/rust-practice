use add_one;

fn main() {
    let result = add_one::add_one(2);
    let n = add_one::add_rand(result as u32);
    println!("{result}, {n}");
}

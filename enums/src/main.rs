#[derive(Debug)]
enum Status {
    Success,
    Failed(String),
}

fn main() {
    let s = Status::Success;

    let f = Status::Failed(String::from("some error"));
    println!("{:?}", s);
    println!("{:?}", f);

    match s {
        Status::Failed(e) => println!("match failed: {}", e),
        // Status::Success => println!("success matched"),
        other => println!("{:?}", other),
    }

}

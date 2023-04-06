use std::fmt::Display;

struct Data<T> {
    value: T,
}

trait Fooer {
    fn foo(&self) {
        println!("default impl foo");
    }
}

impl<T:Display> Fooer for Data<T> {
    fn foo(&self) {
        println!("foo: {}", self.value);
    }
}

struct User<T> (T);
struct Emp (i32);

impl<T> Fooer for Data<User<T>>{}

fn main() {
    let e = Emp(11);
    println!("{}", e.0);

    let d = Data{value: 10};

    let s = Data{value: "dhaval"};

    println!("d is {}", d.value);
    println!("s is {}", s.value);

    let du = Data{value: User(22)};


    s.foo();
    du.foo();
}

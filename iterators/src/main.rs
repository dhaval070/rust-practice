#[derive(Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    name: String,
}

fn find_largest<'a>(shoes: &'a Vec<Shoe>) -> Option<&'a Shoe>{
    if shoes.len() == 0 {
        return None;
    }
    let mut b = &shoes[0];

    for s in shoes.iter() {
        if s.size > b.size {
            b = s;
        }
    }

    return Some(b);
}

fn find_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}


fn main() {
    let ar = vec![1, 2, 3];

    let i = ar.iter();

    for n in i {
        println!("{n}");
    }

    println!("{}", ar[0]);

    let shoes = vec![Shoe{name:"yy".to_string(), size: 10}, Shoe{name: "nike".to_string(),size:10}, Shoe{name: "bata".to_string(), size:11}, Shoe{name: "xx".to_string(),size: 12}];

    if let Some(s) = find_largest(&shoes) {
        println!("{:?}", s);
    }

    let ss = find_size(shoes, 10);
    for s in ss {
        println!("{:?}", s);
    }
}

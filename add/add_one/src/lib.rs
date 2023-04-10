use tinyrand::{StdRand,Rand};

pub fn add_one(num: usize) -> usize {
    return num + 1;
}

pub fn add_rand(num: u32) -> u32 {
    let mut rand = StdRand::default();
    num + rand.next_u32()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}

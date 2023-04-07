pub trait Measure {
    fn area(&self) -> i32;
}

pub struct Square {
    pub s: i32,
}

impl Measure for Square {
    fn area(&self) -> i32 {
        self.s * self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_area() {
        let s = Square {
            s: 10,
        };

        assert_eq!(100, s.area() );
    }

}

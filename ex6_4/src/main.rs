#[derive(Debug)]
struct S {
    pub a: u32,
    pub b: u8,
}

struct T {
    pub v: Vec<S>
}

impl Iterator for T {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            Some(self.v.remove(0))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        let t = T{
            v: vec![
                S{a:1, b:2},
                S{a:3, b:4},
            ]};
        for i in t {
            println!("{:?}", i);
        }
        assert_eq!(0,0);
    }
}

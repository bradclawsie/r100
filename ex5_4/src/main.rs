enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    pub fn radius(&self) -> f64 {
        let Shape::Circle { radius } = self else {
            panic!("not circle");
        };
        *radius
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
        let s: Shape = Shape::Circle { radius : 24.0 };
        assert_eq!(s.radius(), 24.0);
    }

    #[test]
    #[should_panic(expected = "not circle")]
    fn t1() {
        let s: Shape = Shape::Square { border: 1.0 };
        s.radius();
    }
}

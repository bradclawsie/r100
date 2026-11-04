enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    fn n_sides(&self) -> u8 {
        match self {
            Self::Circle => 1,
            Self::Square => 4,
            Self::Rectangle => 4,
            Self::Triangle => 3,
            Self::Pentagon => 5,
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
    fn t0 () {
        assert_eq!(Shape::Circle.n_sides(), 1);
        assert_eq!(Shape::Square.n_sides(), 4);
        assert_eq!(Shape::Rectangle.n_sides(), 4);
        assert_eq!(Shape::Triangle.n_sides(), 3);
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}

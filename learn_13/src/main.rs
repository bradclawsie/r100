#[derive(Debug, PartialEq)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> i64 {
        match self {
            Expr::Number(i) => *i,
            Expr::Add(lhs, rhs) => {
                return lhs.evaluate() + rhs.evaluate();
            },
            Expr::Multiply(lhs, rhs) => {
                return lhs.evaluate() * rhs.evaluate();
            },
            Expr::Negate(i) => {
                return -1 as i64 * i.evaluate();
            },
        }
    }

    fn depth(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            Expr::Add(lhs, rhs) | Expr::Multiply(lhs, rhs)=> {
                return 1 + std::cmp::max(lhs.depth(), rhs.depth());
            },
            Expr::Negate(i) => {
                return 1 + i.depth();
            },
        }
    }

    fn node_count(&self) -> usize {
        match self {
            Expr::Number(_) => 1,
            Expr::Add(lhs, rhs) | Expr::Multiply(lhs, rhs)=> {
                return 1 + lhs.node_count() + rhs.node_count();
            },
            Expr::Negate(i) => {
                return 1 + i.node_count();
            },
        }
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_evaluate() {
        assert_eq!(Expr::Number(1).evaluate(), 1);
        assert_eq!(Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))
        ).evaluate(),
            3);
        assert_eq!(Expr::Multiply(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))).evaluate(),
            2);
        assert_eq!(Expr::Negate(
            Box::new(Expr::Number(1))).evaluate(),
            -1);
        let expr = Expr::Multiply(
            Box::new(Expr::Add(
                Box::new(Expr::Number(2)),
                Box::new(Expr::Number(3)),
            )),
            Box::new(Expr::Negate(
                Box::new(Expr::Number(4)),
            )),
        );
        assert_eq!(expr.evaluate(), -20);
        assert_eq!(expr.evaluate(), -20);
    }

    #[test]
    fn test_depth() {
        assert_eq!(Expr::Number(1).depth(), 1);
        assert_eq!(Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))
        ).depth(),
            2);
        assert_eq!(Expr::Multiply(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))).depth(),
            2);
        assert_eq!(Expr::Negate(
            Box::new(Expr::Number(1))).depth(),
            2);
        let expr = Expr::Multiply(
            Box::new(Expr::Add(
                Box::new(Expr::Number(2)),
                Box::new(Expr::Number(3)),
            )),
            Box::new(Expr::Negate(
                Box::new(Expr::Number(4)),
            )),
        );
        assert_eq!(expr.depth(), 3);
    }

    #[test]
    fn test_node_count() {
        assert_eq!(Expr::Number(1).node_count(), 1);
        assert_eq!(Expr::Add(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))
        ).node_count(),
            3);
        assert_eq!(Expr::Multiply(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2))).node_count(),
            3);
        assert_eq!(Expr::Negate(
            Box::new(Expr::Number(1))).node_count(),
            2);
        let expr = Expr::Multiply(
            Box::new(Expr::Add(
                Box::new(Expr::Number(2)),
                Box::new(Expr::Number(3)),
            )),
            Box::new(Expr::Negate(
                Box::new(Expr::Number(4)),
            )),
        );
        assert_eq!(expr.node_count(), 6);
    }
}

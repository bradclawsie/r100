fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut t = n;
    let mut j = n - 1;
    while j > 0 {
        t = t * j;
        j = j - 1;
    }
    t
}

fn main() {
    println!("{}", factorial(5));
}

#[cfg(test)]
mod tests {
    use crate::factorial;
    
    #[test]
    fn t0 () {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn t1 () {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn t2 () {
        assert_eq!(factorial(5), 120);
    }
}

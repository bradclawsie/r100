fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        return 12;
    }
    if n % 3 == 0 {
        return 13;
    }
    17
}

fn main() {
    println!("{}", magic_number(17));
}

#[cfg(test)]
mod tests {
    use crate::magic_number;
    
    #[test]
    fn t0() {
        assert_eq!(magic_number(4), 12);        
    }

    #[test]
    fn t1() {
        assert_eq!(magic_number(9), 13);        
    }

    #[test]
    fn t2() {
        assert_eq!(magic_number(17), 17);        
    }
}

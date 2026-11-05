#[derive(Debug, PartialEq)]
enum E {
    Err1,
    Err2,
    Err3(String),
}

fn f(i: u32) -> Result<u32, E> {
    if i == 0 {
        return Ok(i);
    }
    if i == 1 {
        return Err(E::Err1);
    }
    if i == 2 {
        return Err(E::Err2);
    }
    if i == 3 {
        return Err(E::Err3(String::from("Err3")));
    }
    Ok(99)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        assert_eq!(f(0 as u32), Ok(0 as u32));
    }

    #[test]
    fn t1() {
        assert_eq!(f(1 as u32), Err(E::Err1));
    }

    #[test]
    fn t2() {
        assert_eq!(f(2 as u32), Err(E::Err2));
    }

    #[test]
    fn t3() {
        assert_eq!(f(3 as u32), Err(E::Err3("Err3".to_string())))
    }
}

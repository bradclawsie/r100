use thiserror;

#[derive(thiserror::Error, Debug, PartialEq)]
enum MyErr {
    #[error("error1")]
    Error1,
    #[error("error2")]
    Error2(String),
}

fn f(i: u32) -> Result<u32, MyErr> {
    if i == 0 {
        return Err(MyErr::Error1);
    }
    if i == 1 {
        return Err(MyErr::Error2("hi".to_string()));
    }
    Ok(i)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use thiserror;
    use crate::*;

    #[test]
    fn t0 () {
        let r = f(0);
        assert_eq!(r, Err(MyErr::Error1));
    }

    #[test]
    fn t1 () {
        let r = f(1);
        assert_eq!(r, Err(MyErr::Error2("hi".to_string())));
    }
}

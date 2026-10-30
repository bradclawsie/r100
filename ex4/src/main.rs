fn velocity(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed == 0 {
        panic!("time_elapsed is zero");
    }
    if end < start {
        panic!("end < start");
    }
    end - start / time_elapsed
}

fn main() {
    println!("{}", velocity(1,2,1));
}

#[cfg(test)]
mod tests {
    use crate::velocity;
    
    #[test]
    #[should_panic(expected="time_elapsed is zero")]
    fn t0 () {
        velocity(1,2,0);
    }

    #[test]
    #[should_panic(expected="end < start")]
    fn t1 () {
        velocity(2,1,1);
    }

    #[test]
    fn t2 () {
        assert_eq!(velocity(1,1,1), 0);
    }
}

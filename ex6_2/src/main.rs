 pub fn fib(i: u32) -> u32 {
     let mut ns: Vec<u32> = vec![0,1];
     if i <= 1 {
         return ns[i as usize];
     }
     for j in 2..i as usize {
         ns.push(ns[j-1] + ns[j-2]);
     }
     ns[(i-1) as usize] + ns[(i-2) as usize]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
    }
}

fn slice_sum(ns: &[u32]) -> u32 {
    ns.iter().sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        assert_eq!(slice_sum(vec![1,2,3].as_slice()), 6);
    }
}

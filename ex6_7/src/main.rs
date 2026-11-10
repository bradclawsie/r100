// instead of following study guide, just try fizzbuzz

fn fb(n: u32) -> Vec<String> {
    if n < 1 {
        panic!("n is less than 1");
    }
    let f = |n: u32| -> String {
        if n % 15 == 0 {
            return "fizzbuzz".to_string();
        }
        if n % 5 == 0 {
            return "fizz".to_string();
        }
        if n % 3 == 0 {
            return "buzz".to_string();
        }
        ToString::to_string(&n)
    };
    (1..n+1).map(|n| f(n)).collect()
}

fn main() {
    println!("{:?}", fb(25));
}

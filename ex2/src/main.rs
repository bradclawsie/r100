fn velocity(start: u32, end: u32, time: u32) -> u32 {
    if end < start {
        panic!("end < start");
    }
    end - start / time
}

fn main() {
    println!("{}", velocity(5,15,4));
}

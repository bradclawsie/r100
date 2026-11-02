#[cfg(test)]
mod tests {
    use ex3_12::order::Order;

    #[test]
    fn t0() {
        assert!(Order::new("p",1,1).is_ok());
    }

    #[test]
    fn t1() {
        assert!(Order::new("",1,1).is_err());
        assert!(Order::new("p",0,1).is_err());
        assert!(Order::new("p",1,0).is_err());
    }
}

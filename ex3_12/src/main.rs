mod order;

use order::Order;

fn main() {
    let mut o = match Order::new("p",1,1) {
        Ok(_o) => _o,
        Err(e) => panic!("{}", e),
    };
    o.set_product_name("p2");
    o.set_quantity(2);
    o.set_unit_price(2);
    println!("{}{}{}{}", o.product_name(), o.quantity(), o.unit_price(), o.total());
}

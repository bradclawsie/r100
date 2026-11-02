pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: &str, quantity: u32, unit_price: u32) -> Result<Self, &'static str> {
        if product_name.is_empty() {
            return Err("product_name is empty");
        }
        if product_name.len() > 300 {
            return Err("product_name len is greater than 300");
        }
        if quantity == 0 {
            return Err("quantity is 0");
        }
        if unit_price == 0 {
            return Err("unit price is 0");
        }
        Ok(Self {
            product_name: String::from(product_name),
            quantity,
            unit_price,
        })
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> String {
        self.product_name.clone()
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn unit_price(&self) -> u32 {
        self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: &str) {
        self.product_name = String::from(product_name);
    }

    pub fn set_quantity(&mut self, quantity:u32) {
        self.quantity = quantity
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        self.unit_price = unit_price
    }
}

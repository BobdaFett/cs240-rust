pub struct CustomerRecord {
    customer_number: i32,
    pin: i32,
    savings_number: i32,
    checking_number: i32
}

impl CustomerRecord {
    pub fn new() -> CustomerRecord {
        CustomerRecord {
            customer_number: 0,
            pin: 0,
            savings_number: 0,
            checking_number: 0
        }
    }

    pub fn from(customer_number: i32, pin: i32) -> CustomerRecord {
        CustomerRecord {
            customer_number,
            pin,
            savings_number: 0,
            checking_number : 0
        }
    }

    // Getters
    pub fn get_number(&self) -> i32 {
        self.customer_number
    }

    pub fn get_pin(&self) -> i32 {
        self.pin
    }

    pub fn get_savings_number(&self) -> i32 {
        self.savings_number
    }

    pub fn get_checking_number(&self) -> i32 {
        self.checking_number
    }

    // Setters
    pub fn set_number(&mut self, customer_number: i32) {
        self.customer_number = customer_number;
    }

    pub fn set_pin(&mut self, pin: i32) {
        self.pin = pin;
    }

    pub fn set_savings_number(&mut self, savings_number: i32) {
        self.savings_number = savings_number;
    }

    pub fn set_checking_number(&mut self, checking_number: i32) {
        self.checking_number = checking_number;
    }
}
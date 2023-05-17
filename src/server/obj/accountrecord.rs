pub struct AccountRecord {
    account_num: i32,
    balance: f32
}

impl AccountRecord {
    pub fn new() -> AccountRecord {
        AccountRecord { account_num: 0, balance: 0.0 }
    }

    pub fn from(account_num: i32, balance: f32) -> AccountRecord {
        AccountRecord { account_num, balance }
    }

    pub fn get_number(&self) -> i32 {
        self.account_num
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance
    }
}
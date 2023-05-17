use AccountRecord;
use CustomerRecord;
use std::io::prelude::*;

pub struct BankData {
    accounts: Vec<AccountRecord>,
    customers: Vec<CustomerRecord>
}

impl BankData {
    pub fn new() -> BankData {
        BankData { accounts: None, customers: None }
    }

    pub fn Init(&mut self) {
        self.accounts = Vec::new();
        self.customers = Vec::new();

        // Manually create accounts.
        for i in 0..10 {
            self.accounts.push(AccountRecord::new(i, 0));
        }

        for i in 0..5 {
            self.customers.push(CustomerRecord::new(i, i));
            self.customers[i].set_checking_number(2 * i);
            self.customers[i].set_savings_number(2 * i + 1);
        }

        println!("Finished data initialization.");
        for record in self.accounts {
            println!("{:?}", record);
        }

        for customer in self.customers {
            println!("{:?}", customer);
        }
    }

    pub fn get_customer(&self, customer_number: i32) -> Option<CustomerRecord> {
        if customer_number < self.customers.len() {
            if self.customers[customer_number].get_number() == customer_number {
                Some(self.customers[customer_number])
            }
        }
        None
    }

    pub fn get_account(&self, account_number: i32) -> Option<AccountRecord> {
        // Find account and return.
        if account_number < self.accounts.len() {
            if self.accounts[account_number].get_number() == account_number {
                Some(self.accounts[account_number])
            }
        }
        None
    }

    pub fn get_balance(&self, account_number: i32) -> Option<f32> {
        // Find account and get balance.
        match (self.get_account(account_number)) {
            Some(account) => { Some(account.get_balance()) },
            None          => { None }
        }
    }

    pub fn get_customer_records(&self) -> Vec<CustomerRecord> {
        customers
    }

    pub fn set_balance(&self, account_number: i32, balance: f32) -> Result<AccountRecord, &'static String> {
        // Find account and then set balance.
        match self.get_account(account_number) {
            Some(account) => {
                account.set_balance(balance);
                Ok(account)
            },
            None => Err("Account does not exit.")
        }
    }
}
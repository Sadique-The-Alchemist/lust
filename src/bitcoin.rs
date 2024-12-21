pub mod bitcoin {
    pub type Bitcoin = i32;

    #[derive(Debug, Copy, Clone)]
    pub struct Wallet {
        balance: Bitcoin,
    }
    impl Wallet {
        pub fn deposite(&mut self, amount: Bitcoin) {
            self.balance += amount
        }
        pub fn balance(self) -> Bitcoin {
            return self.balance;
        }
        pub fn withdraw(&mut self, amount: Bitcoin) -> Result<(), &'static str> {
            if amount > self.balance {
                return Err("cannot withdraw, insufficient funds");
            }
            self.balance -= amount;
            Ok(())
        }
    }
    pub fn new_wallet(balance: Bitcoin) -> Wallet {
        return Wallet { balance: balance };
    }
}

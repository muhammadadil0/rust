mod bank {
    pub struct BankAccount {
        pub owner: String,
        pub balance: u32,
    }

    impl BankAccount {
        pub fn new(owner: String, balance: u32) -> Self {
            Self { owner, balance }
        }

        pub fn deposit(&mut self, amount: u32) {
            self.balance += amount;
        }

        pub fn withdraw(&mut self, amount: u32) -> Result<u32, String> {
            if amount > self.balance {
                Err(String::from("Insufficient balance"))
            } else {
                self.balance -= amount;
                Ok(self.balance)
            }
        }

        pub fn get_balance(&self) -> u32 {
            self.balance
        }
    }
}

#[cfg(test)]
mod test {
    use super::bank::BankAccount;

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(String::from("Adil"), 100);
        account.deposit(50);
        assert_eq!(account.get_balance(), 150);
    }
}

fn main() {
    println!("main called");
}

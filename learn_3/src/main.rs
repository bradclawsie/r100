struct Account {
    id: u32,
    balance: i64,
}

struct Bank {
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Self {
        Self{
            accounts: Vec::new(),
        }
    }

    pub fn add(&mut self, account: Account) {
        if self.accounts.iter().any(|e| e.id == account.id) {
            // account is already added
            return;
        }
        self.accounts.push(account);
    }

    pub fn get(&self, id: u32) -> Option<&Account> {
        self.accounts.iter().find(|&x| x.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|x| x.id == id)
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: i64) -> bool {
        if from == to {
            return false;
        }
        if amount < 1 {
            return false;
        }
        
        let from_read = match self.get(from) {
            Some(account) => account,
            _ => return false,
        };
        if from_read.balance - amount < 0 {
            return false;
        }

        let to_read = match self.get(to) {
            Some(account) => account,
            _ => return false,
        };
        if to_read.balance.checked_add(amount).is_none() {
            return false;
        }

        let Some(from_write) = self.get_mut(from) else {
            return false;
        };
        from_write.balance -= amount; 
        let Some(to_write) = self.get_mut(to) else {
            return false;
        };
        to_write.balance += amount;
        true
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        assert_eq!(Bank::new().accounts.len(), 0)
    }

    #[test]
    fn test_add() {
        let mut bank = Bank::new();
        bank.add(Account{
            id:1,
            balance:1,
        });
        assert_eq!(bank.accounts.len(), 1);
        bank.add(Account{
            id:1,
            balance:1,
        });
        assert_eq!(bank.accounts.len(), 1);
        bank.add(Account{
            id:2,
            balance:1,
        });
        assert_eq!(bank.accounts.len(), 2);
    }

    #[test]
    fn test_get() {
        let mut bank = Bank::new();
        bank.add(Account{
            id:1,
            balance:1,
        });
        assert!(bank.get(1 as u32).is_some());
        assert!(bank.get(2 as u32).is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut bank = Bank::new();
        bank.add(Account{
            id:1,
            balance:1,
        });
        assert!(bank.get_mut(1 as u32).is_some());
        assert!(bank.get_mut(2 as u32).is_none());
    }

    #[test]
    fn test_transfer() {
        let mut bank = Bank::new();
        let from_id: u32 = 1;
        bank.add(Account{
            id:from_id,
            balance:1,
        });
        let to_id: u32 = 2;
        bank.add(Account{
            id:to_id,
            balance:1,
        });

        assert_eq!(bank.transfer(from_id,to_id,1), true);
        assert!(bank.get(from_id).is_some_and(|a| a.balance == 0));
        assert!(bank.get(to_id).is_some_and(|a| a.balance == 2));

        // zero transfers not allowed
        assert_eq!(bank.transfer(from_id,to_id,0), false);

        // negative transfers not allowed
        assert_eq!(bank.transfer(from_id,to_id,-1), false);
        
        // from account depleted
        assert_eq!(bank.transfer(from_id,to_id,1), false);

        // account numbers the same
        assert_eq!(bank.transfer(from_id,from_id,1), false);

        // from account not found
        assert_eq!(bank.transfer(99,to_id,1), false);

        // to account not found
        assert_eq!(bank.transfer(from_id, 99, 1), false);

        let maxed_id: u32 = 3;
        bank.add(Account{
            id:maxed_id,
            balance:i64::MAX,
        });

        // do not let accounts overflow
        assert_eq!(bank.transfer(to_id, maxed_id, 1), false);

    }
    
}

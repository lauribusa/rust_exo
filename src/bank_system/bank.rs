

enum Transation {
    Deposit {
        amount: f64,
        source: String
    },
    Withdrawal {
        amount: f64,
        target: String
    }
}

pub struct BankAccount {
    money: f64,
    active: bool,
    iban: String,
    transactions: Vec<Transation>
}

#[derive(Debug)]
pub enum TransferError {
    NotEnoughMoney(String),
    FatalError(String)
}

pub enum WithdrawError {
    NotEnoughMoney
}

pub enum DepositError {
    AccountInactive
}


impl BankAccount {
    pub fn new(money: f64, active: bool, iban: String) -> BankAccount {
        BankAccount {
            money,
            active,
            iban,
            transactions: vec![]
        }
    }

    pub fn get_money(&self) -> f64 {
        self.money
    }

    fn withdraw(&mut self, amount: f64, withdrawer: &BankAccount) -> Result<(), WithdrawError> {
        if self.money - amount < 0.0 {
            return Err(WithdrawError::NotEnoughMoney);
        }
        self.money -= amount;

        self.transactions.push(
            Transation::Withdrawal { amount, target: withdrawer.iban.clone() }
        );

        Ok(())
    }

    fn deposit(&mut self, amount: f64, depositer: &BankAccount) -> Result<(), DepositError> {
        if !self.active {
            return Err(DepositError::AccountInactive);
        }

        self.money += amount;

        self.transactions.push(
            Transation::Deposit { amount, source: depositer.iban.clone() }
        );

        Ok(())
    }

    pub fn transfer(source: &mut BankAccount, target: &mut BankAccount, amount: f64) -> Result<(), TransferError> {
        source.withdraw(amount, target).map_err(|_| TransferError::NotEnoughMoney(String::from("Pas assez d'argent")))?;
        
        if let Err(_) = target.deposit(amount, source) {
            source.deposit(amount, &target).map_err(|_| TransferError::FatalError(String::new()))?
        }
        // on est heureux
        Ok(())
    }
}
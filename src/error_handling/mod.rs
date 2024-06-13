struct Account {
    name: String,
    account: f64,
    is_active: bool,
}

enum AccountError {
    NotEnoughMoney,
    NotActive
}

impl Account {
    fn new(name: String, is_active: bool) -> Account {
        Account {
            name,
            account: 0 as f64,
            is_active
        }
    }
    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str>{
        let mm = Some(amount);
        if mm.is_none() {
            eprintln!("No value provided");
            return Err("No value provided")
        }

        if (self.account - amount) < 0_f64 {
            eprintln!("Not enough money");
            return Err("Not enough money")
        }
        self.account -= amount;
        Ok(())
    }

    fn deposit(&mut self, amount:f64) -> Result<(), &'static str>{
        let exists = Some(amount);
        if exists.is_none() {
            eprintln!("Need a deposit amount.");
            return Err("Need a deposit amount.")
        }
        if amount < 0_f64 {
            eprintln!("Deposit amount cannot be negative");
            return Err("Deposit amount cannot be negative.")
        }

        self.account += amount;
        Ok(())
    }
}

pub fn banking_system(){
    let mut transfers: Vec<(String, String, f64)> = vec![];

    let mut my_account = Account::new(String::from("Mine"), true);
    my_account.account = 2000_f64;
    let mut their_account = Account::new(String::from("Their"), true);
    let mut disabled_account = Account::new(String::from("Disabled"), false);

    let transaction_result = transfer(&mut my_account, &mut their_account, 64_f64, &mut transfers); 
    if transaction_result.is_err() {
        eprintln!("An error occured with this transaction.");
    }
    let other_result = transfer(&mut my_account, &mut disabled_account, 100_f64, &mut transfers);
    if other_result.is_err() {
        eprintln!("An error occured with the second transaction.");
    }

    let one_more = transfer(&mut my_account, &mut their_account, 200_f64, &mut transfers);
    if one_more.is_err() {
        eprintln!("An error occured with the last transaction.");
    }

    for tuple in transfers {
        println!("Transfer: {} : {} ({})", tuple.0, tuple.1, tuple.2);
    }

    println!("Money: {}, Name: {}, Is active? {}", my_account.account, my_account.name, my_account.is_active);
    println!("Money: {}, Name: {}, Is active? {}", their_account.account, their_account.name, their_account.is_active);
    println!("Money: {}, Name: {}, Is active? {}", disabled_account.account, disabled_account.name, disabled_account.is_active);
}

fn transfer(debitor: &mut Account, beneficiary: &mut Account, amount: f64, transfers: &mut Vec<(String, String, f64)>) -> Result<(), &'static str>{
    let debitor_exists = Some(&debitor);
    let beneficiary_exists = Some(&beneficiary);
    if debitor_exists.is_none() {
        eprintln!("No debitor provided.");
        return Err("No debitor provided.")
    }
    if beneficiary_exists.is_none() {
        eprintln!("No beneficiary provided.");
        return Err("No beneficiary provided.")
    }
    if !debitor.is_active {
        eprintln!("Debitor is not an active account");
        return Err("Debitor is not an active account")
    }
    if !beneficiary.is_active {
        eprintln!("Beneficiary is not an active account");
        return Err("Beneficiary is not an active account")
    }

    let amount_withdrawn = debitor.withdraw(amount);
    if amount_withdrawn.is_err() {
        eprintln!("An error occured during the withdrawal action");
        return Err("An error occured during the withdrawal action.")
    }
    
    let result = beneficiary.deposit(amount);
    if result.is_err() {
        let r = debitor.deposit(amount);
        if r.is_err() {
            panic!("WHAT THE FUCK IS THIS BANK")
        }
        eprintln!("An error occured during the deposit action. The amount has been refunded.");
        return Err("An error occured during the deposit action. The amount has been refunded.")
    }

    transfers.push((debitor.name.to_owned(), beneficiary.name.to_owned(), amount));

    Ok(())
}
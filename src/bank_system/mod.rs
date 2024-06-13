mod bank;

pub fn bank() {
    let mut account_a = bank::BankAccount::new(5000.0, true, String::from("BENOOTNOOT"));
    let mut account_b = bank::BankAccount::new(10000.0, true, String::from("BEFOOFOO"));

    match bank::BankAccount::transfer(&mut account_a, &mut account_b, 5000.0) {
        Ok(_) => {
            println!("OK");
        },
        Err(e) => {
            println!("Erreur: {:?}", e)
        },
    }

    println!("{}", account_a.get_money());
    println!("{}", account_b.get_money());
}
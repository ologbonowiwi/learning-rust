#[derive(Debug)]
struct Owner {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct Account {
    owner: Owner,
    amount: f64,
}

impl Account {
    fn withdraw(&mut self, value: f64) {
        self.amount -= value;
    }
}

fn bank_account() {
    let mut account = Account {
        owner: Owner {
            first_name: String::from("Wesley"),
            last_name: String::from("Matos"),
        },
        amount: 100.0,
    };

    println!(
        "First name = {}, last name = {}, amount = {}",
        account.owner.first_name, account.owner.last_name, account.amount
    );

    account.withdraw(50.0);

    println!("Amount after withdraw = {}", account.amount);
}

pub fn main() {
    bank_account();
}

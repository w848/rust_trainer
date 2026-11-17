use std::cell::RefCell;
use std::rc::Rc;

struct Account {
    owner: String,
    balance: i64,
}

fn open_account(owner: &str) -> Rc<RefCell<Account>> {
    Rc::new(RefCell::new(Account { owner: owner.to_string(), balance: 0 }))
}

fn deposit(account: &Rc<RefCell<Account>>, amount: i64) {
    account.borrow_mut().balance += amount;
}

fn withdraw(account: &Rc<RefCell<Account>>, amount: i64) -> Result<(), String> {
    let mut acc = account.borrow_mut();
    if acc.balance < amount {
        return Err("not enough money".to_string());
    }
    acc.balance -= amount;
    Ok(())
}

fn balance(account: &Rc<RefCell<Account>>) -> i64 {
    account.borrow().balance
}

fn main() {
    let mine = open_account("Mat");
    let my_partners_card = Rc::clone(&mine);
    deposit(&mine, 100);
    println!("{:?}", withdraw(&my_partners_card, 30));
    println!("{:?}", withdraw(&my_partners_card, 500));
    println!("{} has {}", mine.borrow().owner, balance(&mine));
}

use std::cell::RefCell;
use std::rc::Rc;

struct Account {
    owner: String,
    balance: i64,
}

fn open_account(owner: &str) -> Rc<RefCell<Account>> {
    todo!()
}

fn deposit(account: &Rc<RefCell<Account>>, amount: i64) {
    todo!()
}

fn withdraw(account: &Rc<RefCell<Account>>, amount: i64) -> Result<(), String> {
    todo!()
}

fn balance(account: &Rc<RefCell<Account>>) -> i64 {
    todo!()
}

fn main() {
    let mine = open_account("Mat");
    let my_partners_card = Rc::clone(&mine);
    deposit(&mine, 100);
    println!("{:?}", withdraw(&my_partners_card, 30));
    println!("{:?}", withdraw(&my_partners_card, 500));
    println!("{} has {}", mine.borrow().owner, balance(&mine));
}

use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: vec![]
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill)
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}


fn user_input() -> Option<String> { // must return OWNED String.
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter a valid option.");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

mod menu {
    use crate::{user_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match user_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match user_input() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }
    fn show() {
        println!();
        println!("*** BILL MANAGER ***");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!();
        println!("Enter Selection: ");
    }
}

fn main() {
    loop {
        println!("Welcome to EZ Billing");
        MainMenu::show();
        let input = user_input().expect("No Data..");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBill) => (),
            None => return
        }
    }
}

use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false
        }
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

// convert string to f64
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match user_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        // converts the input String to <f64> with the parse().
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        };
    }
}

mod menu {
    use crate::{user_input, get_bill_amount, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match user_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bills(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove");
        let name = match user_input() {
            Some(name) => name,
            None => return
        };
        if bills.remove(&name) {
            println!("BILL HAS BEEN REMOVED.")
        } else {
            println!("No Bills To Remove");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        for bills in bills.get_all() {
            println!("{:?}", bills);
        }
        println!("Enter Bill to Update");
        let name = match user_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Updated!");
        } else {
            println!("Bill not found.")
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }
    fn show() {
        println!();
        println!("*** BILL MANAGER ***");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Update Bills");
        println!("Enter Selection: ");
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let input = user_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bills(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break
        }
    }
    None
}

fn main() {
    run_program();
}

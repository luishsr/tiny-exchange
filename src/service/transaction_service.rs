use System::SystemDate;
use System::SystemTime;

pub enum txType {}

pub struct Transaction {
    id: u32,
    txType: txType,
    amount: f64,
    date: SystemDate,
    time: SystemTime,
}

pub impl Transaction {
    pub fn new_transaction() {}
}

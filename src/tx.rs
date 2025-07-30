// İşlem yapısı (Transaction) ve validasyon.
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: f64,
}

impl Transaction {
    pub fn new(sender: String, reciever: String, amount: f64) -> Self {
        Transaction {
            sender,
            reciever,
            amount,
        }
    }
}

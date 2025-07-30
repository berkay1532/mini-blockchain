mod block;
mod blockchain;
mod tx;

use blockchain::Blockchain;
use tx::Transaction;

fn main() {
    println!("CLI komutlarını parse eder, zinciri oluşturur ve çalıştırır.");

    let mut blockchain = Blockchain::new(4);

    let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 3.12);
    let tx2 = Transaction::new(String::from("Dave"), String::from("Mary"), 21.44);
    let tx_vec = vec![tx1, tx2];

    blockchain.add_block(tx_vec);

    let tx1 = Transaction::new(String::from("Bob"), String::from("Dave"), 1.95);
    let tx2 = Transaction::new(String::from("Mary"), String::from("Alice"), 21.44);
    let tx_vec = vec![tx1, tx2];

    blockchain.add_block(tx_vec);

    let tx1 = Transaction::new(String::from("Aby"), String::from("Beth"), 6.77);
    let tx2 = Transaction::new(String::from("Peter"), String::from("Jay"), 84.2);
    let tx_vec = vec![tx1, tx2];
    blockchain.add_block(tx_vec);

    println!("- - - - - - - Blockhain - - - - - - - - -");

    blockchain.print_chain();

    print!("\n Blockchain geçerli mi?  {}\n", blockchain.is_valid());
}

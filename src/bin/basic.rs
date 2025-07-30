use mini_blockchain::blockchain::Blockchain;
use mini_blockchain::tx::Transaction;

fn main() {
    let mut chain = Blockchain::new(4);

    let tx1 = Transaction::new("Ali".into(), "Veli".into(), 50.0);
    let tx2 = Transaction::new("Zeynep".into(), "Murat".into(), 75.0);

    chain.add_block(vec![tx1, tx2]);
    chain.print_chain();

    println!("Blockchain geçerli mi? {}", chain.is_valid());
}

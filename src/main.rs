mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    println!("CLI komutlarını parse eder, zinciri oluşturur ve çalıştırır.");

    let mut blockchain = Blockchain::new(4);

    blockchain.add_block(String::from("2.blok verimiz"));
    blockchain.add_block(String::from("3.blok verimiz"));
    blockchain.add_block(String::from("3.blok verimiz"));

    println!("- - - - - - - Blockhain - - - - - - - - -");

    blockchain.print_chain();

    print!("\n Blockchain geçerli mi?  {}\n", blockchain.is_valid());
}

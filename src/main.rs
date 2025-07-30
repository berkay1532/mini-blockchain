mod block;
mod blockchain;
mod pow;
mod server;
mod tx;

use blockchain::Blockchain;
use clap::{Parser, Subcommand};
use std::sync::{Arc, Mutex};
use tx::Transaction;

#[tokio::main]

async fn main() {
    let blockchain = Blockchain::load_from_file("blockchain.json", 4);
    let blockchain = Arc::new(Mutex::new(blockchain));

    server::start_server(blockchain).await;
}

const CHAIN_FILE: &str = "blockchain.json";

// Cli
#[derive(Parser)]
#[command(name = "MiniBlockchain")]
#[command(about = "Rust ile yazılmış basit bir blockchain arayüzü", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        from: String,

        #[arg(short, long)]
        to: String,

        #[arg(short, long)]
        amount: f64,
    },

    Show,

    Validate,

    Reset {
        #[arg(short, long)]
        difficulty: usize,
    },
}

// to run from cli
// fn main() {
//     let cli = Cli::parse();
//     // let mut blockchain = Blockchain::new(4);
//     let mut blockchain = Blockchain::load_from_file(CHAIN_FILE, 4);

//     match cli.command {
//         Commands::Add { from, to, amount } => {
//             let tx = Transaction::new(from, to, amount);
//             blockchain.add_block(vec![tx]);
//             blockchain.save_to_file(CHAIN_FILE);
//         }
//         Commands::Show => {
//             println!("- - - - - - - Blockchain - - - - - - -");
//             blockchain.print_chain();
//         }
//         Commands::Validate => {
//             println!("Is blockchain valid ? \n {}", blockchain.is_valid());
//         }
//         Commands::Reset { difficulty } => {
//             blockchain = Blockchain::new(difficulty);
//             blockchain.save_to_file(CHAIN_FILE);
//         }
//     }
// }

// to check with prints

// fn main() {
//     println!("CLI komutlarını parse eder, zinciri oluşturur ve çalıştırır.");

//     let mut blockchain = Blockchain::new(4);

//     let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 3.12);
//     let tx2 = Transaction::new(String::from("Dave"), String::from("Mary"), 21.44);
//     let tx_vec = vec![tx1, tx2];

//     blockchain.add_block(tx_vec);

//     let tx1 = Transaction::new(String::from("Bob"), String::from("Dave"), 1.95);
//     let tx2 = Transaction::new(String::from("Mary"), String::from("Alice"), 21.44);
//     let tx_vec = vec![tx1, tx2];

//     blockchain.add_block(tx_vec);

//     let tx1 = Transaction::new(String::from("Aby"), String::from("Beth"), 6.77);
//     let tx2 = Transaction::new(String::from("Peter"), String::from("Jay"), 84.2);
//     let tx_vec = vec![tx1, tx2];
//     blockchain.add_block(tx_vec);

//     println!("- - - - - - - Blockhain - - - - - - - - -");

//     blockchain.print_chain();

//     print!("\n Blockchain geçerli mi?  {}\n", blockchain.is_valid());
// }

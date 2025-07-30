use clap::{Parser, Subcommand};
use mini_blockchain::blockchain::Blockchain;
use mini_blockchain::tx::Transaction;
use std::sync::{Arc, Mutex};

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
fn main() {
    let cli = Cli::parse();
    // let mut blockchain = Blockchain::new(4);
    let mut blockchain = Blockchain::load_from_file(CHAIN_FILE, 4);

    match cli.command {
        Commands::Add { from, to, amount } => {
            let tx = Transaction::new(from, to, amount);
            blockchain.add_block(vec![tx]);
            blockchain.save_to_file(CHAIN_FILE);
        }
        Commands::Show => {
            println!("- - - - - - - Blockchain - - - - - - -");
            blockchain.print_chain();
        }
        Commands::Validate => {
            println!("Is blockchain valid ? \n {}", blockchain.is_valid());
        }
        Commands::Reset { difficulty } => {
            blockchain = Blockchain::new(difficulty);
            blockchain.save_to_file(CHAIN_FILE);
        }
    }
}

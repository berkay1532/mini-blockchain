use mini_blockchain::{blockchain::Blockchain, server};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let chain = Blockchain::load_from_file("blockchain.json", 4);
    let shared_chain = Arc::new(Mutex::new(chain));
    server::start_server(shared_chain).await;
}

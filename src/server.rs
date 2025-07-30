use std::sync::{Arc, Mutex};
use warp::Filter;

use crate::blockchain::Blockchain;
use crate::tx::Transaction;

pub async fn start_server(blockchain: Arc<Mutex<Blockchain>>) {
    // Arc< > Veriyi birden fazla thread ile erişime açık hale getirir
    // Mutex< > Bu veriyi kilitler ve birden fazla threadin veriyi güncellemesini engeller.Veri aynı zamanda tek thread tarafından güncellenebilir

    // GET /chain → zinciri getir
    let chain_route = warp::path("chain") // endpoint path
        .and(warp::get()) // req type
        .and(with_blockchain(blockchain.clone()))
        .and_then(get_chain);

    // POST /add-tx → işlem gönder
    let tx_route = warp::path("add-tx")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_blockchain(blockchain.clone()))
        .and_then(add_transaction);

    // GET /validate → zincir geçerli mi?
    let validate_route = warp::path("validate")
        .and(warp::get())
        .and(with_blockchain(blockchain.clone()))
        .and_then(validate_chain);

    let routes = chain_route.or(tx_route).or(validate_route);
    println!("🔗 HTTP sunucu başlatıldı: http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// --- Handler Fonksiyonları ---

async fn get_chain(
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let chain = blockchain.lock().unwrap();
    Ok(warp::reply::json(&chain.chain))
}

async fn add_transaction(
    new_tx: Transaction,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut chain = blockchain.lock().unwrap();
    chain.add_block(vec![new_tx]);
    chain.save_to_file("blockchain.json");
    Ok(warp::reply::with_status(
        "✅ İşlem eklendi",
        warp::http::StatusCode::CREATED,
    ))
}

async fn validate_chain(
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let chain = blockchain.lock().unwrap();
    Ok(warp::reply::json(&chain.is_valid()))
}

// Middleware: Blockchain'i route'lara aktar
fn with_blockchain(
    blockchain: Arc<Mutex<Blockchain>>,
) -> impl Filter<Extract = (Arc<Mutex<Blockchain>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || blockchain.clone())
}

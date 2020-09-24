use warp::Filter;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct QuoteRequest {
  amount: u32,
  currency: String,
  destination: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct TransferRequest {
  amount: u32,
  currency: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let accounts = warp::path("accounts")
        .map(|| "40 BTC" ); 

    // POST /quote {"amount": 32000, "currency": "COP", "destination": "Banco Dani"}
    let quote = warp::path("quote")
        .and(warp::body::json())
        .map(|req: QuoteRequest| {
          println!("Hello from quote {:?}!", req );
          //warp::reply::json(&money)
          "this is the response from quote"
        });

    // POST /quote/:toAccountId {"amount": 32000, "currency": "COP"}
    let transfer = warp::path("transfer")
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .map(|toAccountId, mut req: TransferRequest| {
          println!("Hello from transfer {:?}!", req );

          //warp::reply::json(&money)
          "this is the response from transfer"
        });
    
    let routes = warp::get().and(hello.or(accounts)).or(warp::post().and(quote.or(transfer)));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

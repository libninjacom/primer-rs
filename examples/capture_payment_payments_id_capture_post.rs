#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let id = "your id";
    let amount = ::serde_json::json!({});
    let final_ = true;
    let response = client
        .capture_payment_payments_id_capture_post(id, amount, final_)
        .x_idempotency_key("your x idempotency key")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

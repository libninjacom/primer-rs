#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let id = "your id";
    let reason = "your reason";
    let response = client
        .cancel_payment_payments_id_cancel_post(id, reason)
        .x_idempotency_key("your x idempotency key")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

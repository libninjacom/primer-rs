#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let id = "your id";
    let response = client.get_payment_by_id_payments_id_get(id).send().await.unwrap();
    println!("{:#?}", response);
}

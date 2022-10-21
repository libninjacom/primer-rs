#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let customer_id = "your customer id";
    let response = client
        .get_payment_methods_payment_methods_get(customer_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

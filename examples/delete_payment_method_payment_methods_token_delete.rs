#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let payment_method_token = "your payment method token";
    let response = client
        .delete_payment_method_payment_methods_token_delete(payment_method_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

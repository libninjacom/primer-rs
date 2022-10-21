#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let payment_method_token = "your payment method token";
    let customer_id = "your customer id";
    let response = client
        .vault_payment_method_payment_methods_token_vault_post(
            payment_method_token,
            customer_id,
        )
        .verify(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

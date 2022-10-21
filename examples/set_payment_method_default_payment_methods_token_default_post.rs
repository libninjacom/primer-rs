#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let payment_method_token = "your payment method token";
    let response = client
        .set_payment_method_default_payment_methods_token_default_post(
            payment_method_token,
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

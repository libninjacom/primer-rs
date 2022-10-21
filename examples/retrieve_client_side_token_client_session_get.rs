#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let response = client
        .retrieve_client_side_token_client_session_get()
        .client_token("your client token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

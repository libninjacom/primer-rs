#![allow(unused_imports)]
use primer_api::PrimerClient;
use primer_api::model::*;
#[tokio::main]
async fn main() {
    let client = PrimerClient::from_env();
    let id = "your id";
    let resume_token = "your resume token";
    let response = client
        .resume_payment_payments_id_resume_post(id, resume_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

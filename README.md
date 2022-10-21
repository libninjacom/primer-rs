<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/primer-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/primer-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/primer-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/primer-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/primer-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/primer-rs/CI?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/primer-api">
    <img src="https://img.shields.io/crates/d/primer-api?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/primer-api">
    <img src="https://img.shields.io/crates/v/primer-api?style=flat-square" alt="Crates.io" />
</a>

</p>

primer client, generated from the OpenAPI spec.

# Usage

```rust
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

```

This example loads configuration from environment variables, specifically:

* `PRIMER_BASE_URL`

* `PRIMER_API_KEY_AUTH`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
primer-api = "0.1.0"
```


# Documentation



* [Client Library Documentation](https://docs.rs/primer-api)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
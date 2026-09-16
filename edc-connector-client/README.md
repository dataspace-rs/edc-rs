<h1 align="center">EDC-rs</h1>
<div align="center">
  <strong>
    Rust client and tools for <a href="https://github.com/eclipse-edc/Connector">EDC</a>.
  </strong>
</div>

<br />


## edc-connector-client 

A Rust client for [EDC](https://github.com/eclipse-edc/Connector).



### Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
edc-connector-client = "0.1"
```


### Examples


#### Basic usage


Fetching an asset with id `1` and reading the `description` property as string.


```rust
use edc_connector_client::{Auth, EdcConnectorClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EdcConnectorClient::builder()
        .management_url("http://myedc")
        .with_auth(Auth::api_token("password"))
        .build()?;

    let asset = client.assets().get("1").await?;

    println!("Got {:?}", asset);

    println!(
        "Property description: {:?}",
        asset.property::<String>("description").unwrap()
    );

    Ok(())
}
```


#### Management API v5 (EDC-V)

Global v5 resources (`participants`, `participant_configs`, `dataspace_profiles`,
`common_expression_language`, `dcp_scopes`, `cached_documents`,
`schema_validators`) ignore the client's participant context and need a token
with the `management-api:admin` scope; everything else is routed under
`/v5/participants/{participant_context}/...`.

### Development


#### Compiling

```
git clone https://github.com/wolf4ood/edc-rs.git
cd edc-rs
cargo build
```


#### Running Tests

Some tests run against a running instance of EDC.

You can use docker compose to start an instance for testing. 

```
docker compose -f testing/docker-compose.yml up -d
cargo test 
```




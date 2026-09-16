
<div class="oranda-hide">
  <h1 align="center">EDC-rs</h1>
</div>

<div align="center">
  <strong>
    Rust client and tools for <a href="https://github.com/eclipse-edc/Connector">EDC</a>.
  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/dataspace-rs/edc-rs?query=workflow%3ATests">
    <img src="https://github.com/dataspace-rs/edc-rs/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>
  
  <a href="https://crates.io/crates/edc-connector-client">
    <img src="https://img.shields.io/crates/d/edc-connector-client.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/edc-connector-client">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

   <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"
      alt="license" />
  </a>

   <a href="https://deps.rs/repo/github/dataspace-rs/edc-rs">
    <img src="https://deps.rs/repo/github/dataspace-rs/edc-rs/status.svg"
      alt="license" />
  </a>

</div>


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


#### Authentication

The client supports these mechanisms via `Auth`:

- `Auth::api_token("...")` sends the token in the `X-Api-Key` header.
- `Auth::bearer_token("...")` sends a fixed `Authorization: Bearer` token.
- `Auth::oauth(OAuth2Config)` obtains a token with the OAuth2 client credentials grant and refreshes it when it expires.
- `Auth::token_exchange(TokenExchangeConfig)` exchanges a workload credential (e.g. a projected Kubernetes
  ServiceAccount token) at an RFC 8693 token exchange broker for a short-lived scoped token, as described in the
  [JAD token exchange spec](https://github.com/eclipse-dataspace-hub/jad/blob/main/docs/token-exchange.md).
  The exchanged token is cached until it expires and the credential file is re-read on every exchange, so
  rotation is picked up. No client credentials are sent: the subject token is the sole proof of identity.

```rust
use edc_connector_client::{Auth, EdcConnectorClient, SubjectToken, TokenExchangeConfig};

let auth = Auth::token_exchange(
    TokenExchangeConfig::builder()
        .token_exchange_url("http://jwtlet.edc-v.svc.cluster.local:8080/token")
        .subject_token(SubjectToken::file("/var/run/secrets/jwtlet/token"))
        .resource("<participant-context-id>") // becomes the `sub` claim
        .audience("edcv") // default
        .scopes(vec!["management-api:assets:read".to_string()])
        .build(),
)?;

let client = EdcConnectorClient::builder()
    .management_url("http://myedc")
    .with_auth(auth)
    .participant_context("<participant-context-id>")
    .build()?;
```


#### Creating an asset with dataplane metadata

`dataplaneMetadata` replaces the deprecated `dataAddress` on assets (EDC 0.18+). Labels and
profiles are used to select a data plane, while properties are handed over to it.

```rust
use edc_connector_client::types::asset::{DataplaneMetadata, NewAsset};

let asset = NewAsset::builder()
    .id("1")
    .property("description", "An asset")
    .dataplane_metadata(
        DataplaneMetadata::builder()
            .label("http")
            .profile("http-profile")
            .property("baseUrl", "https://example.com/data")
            .build(),
    )
    .build();

let response = client.assets().create(&asset).await?;
```


#### Management API v5 (EDC-V)

Calls go through `client.<api>(EdcConnectorApiVersion::V5)`. Participant scoped
resources (assets, policies, contract definitions, negotiations, agreements,
transfers, catalog, data planes, discovery) are routed under
`/v5/participants/{participant_context}/...` when the client is built with
`.participant_context(..)`. Global resources ignore the participant context and
need a token with the `management-api:admin` scope (profiles can be read with
`management-api:profiles:read`):

| Accessor | Resource |
|---|---|
| `participants(V5)` | `/participants` (create, get, list, update, delete, profiles) |
| `participant_configs(V5)` | `/participants/{id}/config` (get, save, patch) |
| `dataspace_profiles(V5)` | `/dataspaceprofiles` |
| `common_expression_language(V5)` | `/celexpressions` (incl. `test`) |
| `dcp_scopes(V5)` | `/dcpscopes` |
| `cached_documents(V5)` | `/cacheddocuments` (incl. `refresh`) |
| `schema_validators(V5)` | `/schemavalidators` |

```rust
use edc_connector_client::{types::participants::ParticipantContextConfigPatch, EdcConnectorApiVersion};

let participants = client.participants(EdcConnectorApiVersion::V5).list(0, 50).await?;

client
    .participant_configs(EdcConnectorApiVersion::V5)
    .patch(
        "my-participant",
        &ParticipantContextConfigPatch::builder()
            .entry("edc.participant.id", "my-participant")
            .remove_entry("obsolete.key")
            .build(),
    )
    .await?;
```

### Development


#### Compiling

```
git clone https://github.com/dataspace-rs/edc-rs.git
cd edc-rs
cargo build
```


#### Testing 

Some tests run against a running instance of EDC.

You can use docker compose to start an instance for testing. 

```
docker compose -f testing/docker-compose.yml up -d
cargo test 
```

The tests setup was mostly derived by the Typescript client [edc-connector-client](https://github.com/Think-iT-Labs/edc-connector-client)


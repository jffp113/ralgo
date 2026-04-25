use std::marker::PhantomData;
use reqwest::{header, ClientBuilder};
use serde::__private228::de::IdentifierDeserializer;
use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/indexer_codegen.rs"));

const AUTHORIZATION_HEADER: &str = "X-Indexer-API-Token";

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("invalid auth token")]
    InvalidAuthToken,

    #[error("invalid header")]
    InvalidHeader,

    #[error("error initializing client")]
    InitializingClient
}

type Result<T> = std::result::Result<T, IndexerError>;

struct NoUrl;
struct WithUrl;

pub struct IndexerClientBuilder<T> {
    inner: Inner,
    _state: PhantomData<T>
}

struct Inner {
    base_url: String,
    api_token: Option<String>,
}

impl<T> IndexerClientBuilder<T> {

    #[must_use]
    pub fn new() -> IndexerClientBuilder<NoUrl> {
        IndexerClientBuilder{
            inner: Inner{
                base_url: String::new(),
                api_token: None,
            },
            _state: PhantomData,
        }
    }


    pub fn api_token(mut self, api_token: &str) -> Self {
        self.inner.api_token = Some(api_token.to_string());
        self
    }

    //fn header(mut self, key: &str, value: &str) -> Result<Self> {}
}

impl IndexerClientBuilder<NoUrl> {
    pub fn base_url(mut self, base_url: &str) -> IndexerClientBuilder<WithUrl> {
        self.inner.base_url = base_url.to_string();
        IndexerClientBuilder{
            inner: self.inner,
            _state: PhantomData,
        }
    }
}

impl IndexerClientBuilder<WithUrl> {
    pub fn build(self) -> Result<IndexerClient> {
        let mut headers = header::HeaderMap::with_capacity(1);

        if let Some(api_token) = self.inner.api_token {
            let mut value = header::HeaderValue::from_str(&api_token)
                .map_err(|_| {IndexerError::InvalidAuthToken})?;
            value.set_sensitive(true);

            headers.insert(AUTHORIZATION_HEADER, value);
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|_| {IndexerError::InitializingClient})?;


        Ok(IndexerClient{
            client: Client::new_with_client(&self.inner.base_url,client),
        })
    }
}


pub struct IndexerClient {
    client: Client
}

impl IndexerClient {

}

/*
Good progress. Here's a full audit:

  ---
  Fixed since last review

  - Inner struct pattern — correct
  - base_url now sets the value properly
  - Incomplete test method removed

  ---
  Remaining issues

  1. base_url is still on impl<T> — it should only exist on impl IndexerClientBuilder<NoUrl>

  Right now base_url can be called on a WithUrl builder, overwriting it. Move it:
  impl IndexerClientBuilder<NoUrl> {
      pub fn base_url(self, base_url: &str) -> IndexerClientBuilder<WithUrl> { ... }
  }

  2. api_token is on impl<T> but returns Self — broken for NoUrl

  Calling .api_token() on a NoUrl builder returns IndexerClientBuilder<NoUrl>, which is correct, but the intent is it's only meaningful before build(). This is fine if intentional,
  but make it explicit by moving it to a shared trait or at minimum documenting the order. No strong objection here, just be aware.

  3. new() is on impl<T> but constructs IndexerClientBuilder<NoUrl>

  Move it to impl IndexerClientBuilder<NoUrl>:
  impl IndexerClientBuilder<NoUrl> {
      pub fn new() -> Self { ... }
  }

  4. All methods are still private — unusable outside this module

  new, base_url, api_token, build all need pub.

  5. "".to_string() sentinel for base_url in NoUrl state

  The NoUrl state should never need a valid base_url. Use String::new() (cheaper, clearer intent than "".to_string()), or make Inner.base_url an Option<String> resolved only at
  build().

  6. PhantomData::default() → PhantomData

  _state: PhantomData  // ZST, no method call needed

  7. Unused import still present
  use serde::__private228::de::IdentifierDeserializer;  // remove

  8. headers field in Inner is unused

  Inner stores headers but build() creates a fresh HeaderMap and ignores self.inner.headers. Either remove it from Inner or implement the commented-out header() method and use
  self.inner.headers in build().

  9. map_err closures have unnecessary braces
  .map_err(|_| IndexerError::InvalidAuthToken)?   // not |_| { ... }

  ---
  What a world-class builder also includes

  ┌──────────────────────────────────────────┬──────────────────────┐
  │                 Feature                  │        Status        │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Typestate NoUrl/WithUrl                  │ done                 │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Inner struct for field reuse             │ done                 │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ pub visibility on all API methods        │ done                 │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Default impl for the builder             │ missing              │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Custom headers support (header() method) │ done                 │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Display on error variants                │ done (via thiserror) │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ #[must_use] on builder and build()       │ done                 │
  ├──────────────────────────────────────────┼──────────────────────┤
  │ Validate URL format at base_url() call   │ missing              │
  └──────────────────────────────────────────┴──────────────────────┘

  The two highest-value additions are #[must_use] (prevents silently dropped builders) and moving new()/base_url() to impl IndexerClientBuilder<NoUrl> to properly enforce the
  typestate contract.
*/
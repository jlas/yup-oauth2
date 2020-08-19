use crate::error::Error;
use crate::types::TokenInfo;
use hyper::{Body, Request};

/// foo
pub struct MetadataServerFlow;

/// bar
impl MetadataServerFlow {
    /// asdf
    pub(crate) fn new() -> MetadataServerFlow {
        MetadataServerFlow {}
    }
    pub(crate) async fn token<'b, C, I, T>(
        &self,
        hyper_client: &hyper::Client<C>,
        _scopes: I,
    ) -> Result<TokenInfo, Error>
    where
        T: AsRef<str> + 'b,
        C: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
        I: IntoIterator<Item = &'b T>,
    {
        log::debug!("DefaultApplicationCredentials: checking metadata server...");
        let error;
        let uri = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
        let req = Request::builder()
            .uri(uri)
            .header("Metadata-Flavor", "Google")
            .body(Body::empty())
            .unwrap();
        let response = hyper_client.request(req).await;
        match response {
            Ok(response) => {
                let (head, body) = response.into_parts();
                let body = hyper::body::to_bytes(body).await?;
                log::debug!("Received response; head: {:?} body: {:?}", head, body);
                return Ok(TokenInfo::from_json(&body)?);
            }
            Err(new_error) => error = new_error,
        }
        Err(error.into())
    }
}

//header! { (MetadataFlavor, "Metadata-Flavor") => [String] }

use yup_oauth2::authenticator::MetadataServerAuthenticator;

#[tokio::main]
async fn main() {
    let md = MetadataServerAuthenticator::builder()
        .build()
        .await
        .unwrap();
    let scopes = &["https://www.googleapis.com/auth/bigquery"];
    match md.token(scopes).await {
        Err(e) => println!("error: {:?}", e),
        Ok(t) => println!("The token is {:?}", t),
    }
}

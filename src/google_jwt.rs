use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use serde_urlencoded;

pub(crate) fn generate_assertion_body(service_json: String) -> Result<String, anyhow::Error> {
    let service_json: ServiceAccountInfoJson = serde_json::from_str(&service_json)?;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Claims {
        iss: String,
        scope: String,
        aud: String,
    }
    let claims = Claims {
        iss: service_json.client_email,
        scope: "https://www.googleapis.com/auth/bigquery.insertdata".to_string(),
        aud: service_json.token_uri.clone(),
    };

    let key = RS256KeyPair::from_pem(service_json.private_key.as_str())?;

    let claims =
        jwt_simple::claims::Claims::with_custom_claims::<Claims>(claims, Duration::from_secs(3600));

    let assertion = key.sign::<Claims>(claims).unwrap();

    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &assertion),
    ];

    Ok(serde_urlencoded::to_string(params)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ServiceAccountInfoJson {
    pub(crate) project_id: String,
    pub(crate) private_key_id: String,
    pub(crate) private_key: String,
    pub(crate) client_email: String,
    pub(crate) client_id: String,
    pub(crate) auth_uri: String,
    pub(crate) token_uri: String,
    pub(crate) client_x509_cert_url: String,
}

use chrono;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(crate) struct Claims {
    pub(crate) iss: String,
    pub(crate) scope: String,
    pub(crate) aud: String,
}

impl Claims {
    pub(crate) fn new(iss: String, scope: String, aud: String) -> Self {
        Self { iss, scope, aud }
    }
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

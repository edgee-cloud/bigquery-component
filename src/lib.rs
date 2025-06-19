use crate::exports::edgee::components::data_collection::{
    AuthRequest, Dict, EdgeeRequest, Event, HttpMethod,
};
use anyhow::Context;
use exports::edgee::components::data_collection::Guest;
mod google_jwt;
use jwt_simple::prelude::*;
use serde_urlencoded;
use std::collections::HashMap;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all,     additional_derives: [serde::Serialize]});
export!(Component);

struct Component;

impl Component {
    fn event(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let url = format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{}/datasets/{}/tables/{}/insertAll",
            settings.project_id, settings.dataset_id, settings.table_id
        );

        // CREATE TABLE edgee (
        //    uuid UUID,
        //    event_type String,
        //    timestamp UInt64,
        //    timestamp_millis UInt64,
        //    timestamp_micros UInt64,
        //    consent Nullable(String),
        //    context JSON,
        //    data JSON
        //
        // this is serializable
        #[derive(serde::Serialize)]
        struct Body {
            rows: Vec<Row>,
        }

        #[derive(serde::Serialize)]
        struct Row {
            json: Event,
        }

        let body = Body {
            rows: vec![Row { json: edgee_event }],
        };

        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url,
            headers: vec![
                (
                    "Authorization".to_string(),
                    format!("Bearer {}", settings.access_token),
                ),
                ("Content-Type".to_string(), "application/json".to_string()),
            ],
            body: serde_json::to_string(&body).unwrap(),
            forward_client_headers: false,
        })
    }
}
impl Guest for Component {
    #[allow(unused_variables)]
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    #[allow(unused_variables)]
    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    #[allow(unused_variables)]
    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    #[allow(unused_variables)]
    fn authenticate(settings_dict: Dict) -> Result<Option<AuthRequest>, String> {
        let settings = AuthSettings::new(settings_dict).map_err(|e| e.to_string())?;

        let service_json: google_jwt::ServiceAccountInfoJson =
            serde_json::from_str(&settings.service_json)
                .context("Failed to parse service_json")
                .unwrap();

        // https://developers.google.com/identity/protocols/oauth2/service-account#authorizingrequests
        let claims = google_jwt::Claims::new(
            service_json.client_email,
            "https://www.googleapis.com/auth/bigquery.insertdata".to_string(),
            service_json.token_uri.clone(),
        );

        let key = RS256KeyPair::from_pem(service_json.private_key.as_str()).unwrap();

        let claims =
            Claims::with_custom_claims::<google_jwt::Claims>(claims, Duration::from_secs(3600));

        let assertion = key.sign::<google_jwt::Claims>(claims).unwrap();

        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &assertion),
        ];

        let body = serde_urlencoded::to_string(params).unwrap();

        Ok(Some(AuthRequest {
            method: HttpMethod::Post,
            url: "https://oauth2.googleapis.com/token".to_string(),
            headers: vec![(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            )],
            body,
            token_duration: 3600,
            response_token_property_name: Some("access_token".to_string()),
            component_token_setting_name: "access_token".to_string(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct AuthSettings {
    pub service_json: String,
}

impl AuthSettings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let service_json = settings_map
            .get("service_json")
            .context("Missing client_id setting")?
            .to_string();

        Ok(Self { service_json })
    }
}
#[derive(Debug, Clone)]
pub struct Settings {
    pub access_token: String,
    pub dataset_id: String,
    pub project_id: String,
    pub table_id: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let access_token = settings_map
            .get("access_token")
            .context("Missing access_token setting")?
            .to_string();
        let dataset_id = settings_map
            .get("dataset_id")
            .context("Missing dataset_id setting")?
            .to_string();
        let project_id = settings_map
            .get("project_id")
            .context("Missing project_id setting")?
            .to_string();
        let table_id = settings_map
            .get("table_id")
            .context("Missing table_id setting")?
            .to_string();

        Ok(Self {
            access_token,
            dataset_id,
            project_id,
            table_id,
        })
    }
}

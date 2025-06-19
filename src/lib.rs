use crate::exports::edgee::components::data_collection::{
    AuthRequest, Dict, EdgeeRequest, Event, HttpMethod,
};
use anyhow::Context;
use exports::edgee::components::data_collection::Guest;
mod google_jwt;
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

        #[derive(serde::Serialize)]
        struct Body {
            rows: Vec<Row>,
        }

        #[derive(serde::Serialize)]
        struct Row {
            json: RowValue,
        }

        #[derive(serde::Serialize)]
        struct RowValue {
            uuid: String,
            event_type: String,
            timestamp: i64,
            timestamp_millis: i64,
            timestamp_micros: i64,
            consent: Option<String>,
            context: String,
            data: String,
        }

        let body = Body {
            rows: vec![Row {
                json: RowValue {
                    uuid: edgee_event.uuid.to_string(),
                    event_type: match edgee_event.event_type {
                        exports::edgee::components::data_collection::EventType::Page => {
                            "page".to_string()
                        }
                        exports::edgee::components::data_collection::EventType::Track => {
                            "track".to_string()
                        }
                        exports::edgee::components::data_collection::EventType::User => {
                            "user".to_string()
                        }
                    },
                    timestamp: edgee_event.timestamp,
                    timestamp_millis: edgee_event.timestamp_millis,
                    timestamp_micros: edgee_event.timestamp_micros,
                    consent: match edgee_event.consent {
                        Some(consent) => match consent {
                            exports::edgee::components::data_collection::Consent::Granted => {
                                Some("granted".to_string())
                            }
                            exports::edgee::components::data_collection::Consent::Denied => {
                                Some("denied".to_string())
                            }
                            exports::edgee::components::data_collection::Consent::Pending => {
                                Some("pending".to_string())
                            }
                        },
                        None => None,
                    },
                    context: serde_json::to_string(&edgee_event.context)
                        .expect("Failed to serialize context"),
                    data: serde_json::to_string(&edgee_event.data)
                        .expect("Failed to serialize data"),
                },
            }],
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
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        Self::event(edgee_event, settings_dict)
    }

    #[allow(unused_variables)]
    fn authenticate(settings_dict: Dict) -> Result<Option<AuthRequest>, String> {
        let settings = AuthSettings::new(settings_dict).map_err(|e| e.to_string())?;

        let Ok(body) = google_jwt::generate_assertion_body(settings.service_json) else {
            return Err("Failed to generate assertion body".to_string());
        };

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
            .context("Missing service_json setting")?
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

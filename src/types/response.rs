use serde::Serialize;

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SendEmailResponse {
    success: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    message_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
}

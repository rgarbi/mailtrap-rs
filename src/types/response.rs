use serde::Serialize;

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SendEmailResponse {
    pub success: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

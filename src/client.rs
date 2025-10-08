use std::time::Duration;
use url::Url;
use reqwest::{Client, Error};
use crate::types::email::Message;
use crate::types::response::SendEmailResponse;

const SEND_EMAIL_BASE_PATH: &str = "/api/send";

#[derive(Clone)]
pub struct MailtrapClient {
    base_url: Url,
    api_token: String,
    http_client: Client,
}

impl MailtrapClient {
    pub fn new(base_url: &str, api_token: String, timeout: Duration) -> Result<MailtrapClient, String> {
        return match Url::parse(base_url) {
            Ok(url) => {
                Ok(MailtrapClient {
                    base_url: url,
                    api_token,
                    http_client: Client::builder().timeout(timeout).connection_verbose(true).build().unwrap(),
                })
            }
            Err(err) => {
                Err(format!("Could not parse URL. {}", err))
            }
        };
    }

    pub async fn send_email(&self, message: Message) -> Result<SendEmailResponse, Error> {
        let address = format!("{}{}", self.base_url, SEND_EMAIL_BASE_PATH);

        let response = self.http_client.post(address).header("Api-Token", self.api_token.clone()).body(message.to_json()).send().await?.error_for_status();

        return match response {
            Ok(response) => {
                let response_body = response.text().await.unwrap();
                let send_email_response: SendEmailResponse =
                    serde_json::from_str(response_body.as_str()).unwrap();
                return Ok(send_email_response);
            }
            Err(err) => {
                Err(err)
            }
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client() {
        assert!(MailtrapClient::new("https://www.google.com", "api token".to_string(), Duration::from_secs(10)).is_ok());
    }

    #[test]
    fn new_client_parse_error() {
        assert!(MailtrapClient::new("google.com", "api token".to_string(), Duration::from_secs(10)).is_err());
    }
}
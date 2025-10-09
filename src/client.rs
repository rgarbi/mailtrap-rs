use crate::types::email::Message;
use crate::types::response::SendEmailResponse;
use reqwest::{Client, Error};
use std::time::Duration;
use url::Url;

const SEND_EMAIL_BASE_PATH: &str = "api/send";

#[derive(Clone)]
pub struct MailtrapClient {
    base_url: Url,
    api_token: String,
    http_client: Client,
}

impl MailtrapClient {
    pub fn new(
        base_url: &str,
        api_token: String,
        timeout: Duration,
    ) -> Result<MailtrapClient, String> {
        return match Url::parse(base_url) {
            Ok(url) => Ok(MailtrapClient {
                base_url: url,
                api_token,
                http_client: Client::builder()
                    .timeout(timeout)
                    .connection_verbose(true)
                    .build()
                    .unwrap(),
            }),
            Err(err) => Err(format!("Could not parse URL. {}", err)),
        };
    }

    pub async fn send_email(&self, message: Message) -> Result<SendEmailResponse, Error> {
        let address = format!("{}{}", self.base_url, SEND_EMAIL_BASE_PATH);

        let message_body = message.to_json();
        println!("{}", message_body.clone());

        let response = self
            .http_client
            .post(address)
            .header("Api-Token", self.api_token.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(message_body)
            .send()
            .await?
            .error_for_status();

        return match response {
            Ok(response) => {
                let response_body = response.text().await.unwrap();
                let send_email_response: SendEmailResponse =
                    serde_json::from_str(response_body.as_str()).unwrap();
                return Ok(send_email_response);
            }
            Err(err) => Err(err),
        };
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};
    use wiremock::matchers::{header, header_exists, method, path};
    use uuid::Uuid;
    use crate::types::email::{Body, EmailAddress};
    use crate::types::response::SendEmailResponse;
    use super::*;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let body = request.body.clone();
            let email_request: Message =
                serde_json::from_str(String::from_utf8(body).unwrap().as_str()).unwrap();

            let has_from_email: bool = !email_request.from.email.to_string().is_empty();
            let has_subject: bool = !email_request.subject.is_empty();
            has_from_email && has_subject
        }
    }

    #[test]
    fn new_client() {
        assert!(
            MailtrapClient::new(
                "https://www.google.com",
                "api token".to_string(),
                Duration::from_secs(10),
            )
                .is_ok()
        );
    }

    #[test]
    fn new_client_parse_error() {
        assert!(
            MailtrapClient::new(
                "google.com",
                "api token".to_string(),
                Duration::from_secs(10),
            )
                .is_err()
        );
    }

    #[tokio::test]
    async fn send_a_test_email() {
        let mock_server = MockServer::start().await;
        let mailtrap_client_result = MailtrapClient::new(
            mock_server.uri().as_str(),
            "123".to_string(),
            Duration::from_secs(30));

        let mock_response = SendEmailResponse {
            success: true,
            message_ids: Vec::from([Uuid::new_v4().to_string()]),
            errors: Vec::new(),
        };

        let mock_response_json = serde_json::to_string(&mock_response).unwrap();

        Mock::given(header_exists("Api-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("api/send"))
            .and(method("POST".to_string()))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200).append_header("Content-Type", "application/json").set_body_raw(mock_response_json, "application/json"))
            .expect(1)
            .mount(&mock_server)
            .await;

        assert_ok!(mailtrap_client_result.clone());
        let mailtrap_client = mailtrap_client_result.unwrap();

        let address_result = EmailAddress::new("testemail@gmail.com".to_string(), Some("Tester".to_string()));
        assert_ok!(address_result.clone());

        let body: Body = Body::TextAndHtml { text: "Sample Body".to_string(), html: "<html><h1>Sample Body</h1></html>".to_string() };


        let message = Message::new(address_result.clone().unwrap(), "Test".to_string(), body)
            .to(EmailAddress::new("totestemail@gmail.com".to_string(), Some("Tester".to_string())).unwrap())
            .reply_to(address_result.unwrap());

        let send_result = mailtrap_client.send_email(message).await;
        assert_ok!(send_result);
    }
}

use mime::Mime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<EmailAddress>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<EmailAddress>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EmailAddress>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom_variables: HashMap<String, String>,

    pub subject: String,

    #[serde(flatten)]
    pub body: Body,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Body {
    Text { text: String },
    Html { html: String },
    TextAndHtml { text: String, html: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Disposition {
    Inline,
    Attachment,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Attachment {
    pub content: String,

    #[serde(rename = "type")]
    pub content_type: ValidMime,

    pub filename: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disposition: Option<Disposition>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

impl Attachment {
    pub fn new(
        content: String,
        content_type: ValidMime,
        filename: String,
        disposition: Option<Disposition>,
        content_id: Option<String>,
    ) -> Self {
        Self {
            content,
            content_type,
            filename,
            disposition,
            content_id,
        }
    }
}

#[derive(Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress {
    pub email: ValidEmail,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EmailAddress {
    pub fn new(email: String, name: Option<String>) -> Result<Self, String> {
        Ok(Self {
            email: ValidEmail::parse(email)?,
            name,
        })
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ValidMime(String);

impl ValidMime {
    pub fn parse(s: String) -> Result<Self, String> {
        match Mime::from_str(s.as_str()) {
            Ok(mime_string) => Ok(Self(mime_string.to_string())),
            Err(_) => Err(format!("{s} is not a valid Mime Type.")),
        }
    }
}

impl AsRef<str> for ValidMime {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ValidMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ValidMime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(String::from(s)) {
            Ok(mime) => Ok(mime),
            Err(_) => Err(()),
        }
    }
}

impl TryFrom<String> for ValidMime {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for ValidMime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValidMime::parse(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ValidEmail(String);

impl ValidEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validator::ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{s} is not a valid email."))
        }
    }
}

impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ValidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for ValidEmail {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for ValidEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValidEmail::parse(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use claims::{assert_err, assert_ok};
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use serde_json::{Value, json};

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email: String = SafeEmail().fake();
        assert_ok!(ValidEmail::parse(email));
    }

    #[test]
    fn invalid_emails_are_rejected() {
        for s in ["", "noatsymbol.com", "@domain.com", "bad@@example.com"] {
            assert_err!(ValidEmail::parse(s.to_string()), "case {s}");
        }
    }

    #[test]
    fn email_address_new_accepts_optional_name() {
        let e = EmailAddress::new("john@example.com".into(), None).unwrap();
        assert!(e.name.is_none());
    }

    #[test]
    fn attachment_serializes_with_type_key() {
        let a = Attachment::new(
            "ZmlsZQ==".into(),
            "text/html".parse::<ValidMime>().unwrap(),
            "index.html".into(),
            Some(Disposition::Attachment),
            None,
        );
        let v = serde_json::to_value(&a).unwrap();
        assert_eq!(v["type"], Value::String("text/html".into()));
        assert!(v.get("content_id").is_none()); // elided when None
    }

    #[test]
    fn cc_bcc_headers_are_elided_when_empty() {
        let msg = Message {
            from: EmailAddress::new("sales@example.com".into(), Some("Sales".into())).unwrap(),
            to: vec![EmailAddress::new("john@example.com".into(), Some("John".into())).unwrap()],
            cc: vec![],
            bcc: vec![],
            reply_to: None,
            attachments: vec![],
            headers: HashMap::new(),
            custom_variables: HashMap::new(),
            subject: "Hello".into(),
            body: Body::Text { text: "Hi".into() },
            category: None,
        };
        let s = serde_json::to_string(&msg).unwrap();
        assert!(!s.contains("\"cc\""));
        assert!(!s.contains("\"bcc\""));
        assert!(!s.contains("\"headers\""));
        assert!(!s.contains("\"custom_variables\""));
    }

    #[test]
    fn deserialize_mailtrap_example_json() {
        let raw = r#"
        {
          "to": [{"email": "john_doe@example.com", "name": "John Doe"}],
          "cc": [{"email": "jane_doe@example.com", "name": "Jane Doe"}],
          "bcc": [{"email": "james_doe@example.com", "name": "Jim Doe"}],
          "from": {"email": "sales@example.com", "name": "Example Sales Team"},
          "reply_to": {"email": "reply@example.com", "name": "Reply"},
          "attachments": [{
            "content": "PCFET0NUWVBFIGh0bWw+CjxodG1sIGxhbmc9ImVuIj4KCiAgICA8aGVhZD4KICAgICAgICA8bWV0YSBjaGFyc2V0PSJVVEYtOCI+CiAgICAgICAgPG1ldGEgaHR0cC1lcXVpdj0iWC1VQS1Db21wYXRpYmxlIiBjb250ZW50PSJJRT1lZGdlIj4KICAgICAgICA8bWV0YSBuYW1lPSJ2aWV3cG9ydCIgY29udGVudD0id2lkdGg9ZGV2aWNlLXdpZHRoLCBpbml0aWFsLXNjYWxlPTEuMCI+CiAgICAgICAgPHRpdGxlPkRvY3VtZW50PC90aXRsZT4KICAgIDwvaGVhZD4KCiAgICA8Ym9keT4KCiAgICA8L2JvZHk+Cgo8L2h0bWw+Cg==",
            "filename": "index.html",
            "type": "text/html",
            "disposition": "attachment"
          }],
          "custom_variables": {"user_id": "45982", "batch_id": "PSJ-12"},
          "headers": {"X-Message-Source": "dev.mydomain.com"},
          "subject": "Your Example Order Confirmation",
          "text": "Congratulations on your order no. 1234",
          "category": "API Test"
        }
        "#;

        let msg: Message = serde_json::from_str(raw).expect("should parse");
        assert_eq!(msg.subject, "Your Example Order Confirmation");
        // Flattened body captured as Text variant
        match msg.body {
            Body::Text { ref text } => assert!(text.contains("order no. 1234")),
            _ => panic!("expected Body::Text"),
        }
        // Attachment content decodes from base64
        let html_b64 = &msg.attachments[0].content;
        let decoded = STANDARD.decode(html_b64.as_str()).expect("base64 decodes");
        let s = String::from_utf8(decoded).unwrap();
        assert!(s.contains("<html"));
    }

    #[test]
    fn serializes_text_and_html_body_correctly() {
        let msg = Message {
            from: EmailAddress::new("sales@example.com".into(), None).unwrap(),
            to: vec![EmailAddress::new("john@example.com".into(), None).unwrap()],
            cc: vec![],
            bcc: vec![],
            reply_to: None,
            attachments: vec![],
            headers: HashMap::new(),
            custom_variables: HashMap::new(),
            subject: "Hi".into(),
            body: Body::TextAndHtml {
                text: "T".into(),
                html: "<b>H</b>".into(),
            },
            category: None,
        };
        let v = serde_json::to_value(&msg).unwrap();
        assert_eq!(v["text"], "T");
        assert_eq!(v["html"], "<b>H</b>");
    }

    #[test]
    fn invalid_email_in_json_fails_to_deserialize() {
        let bad = json!({
            "from": {"email": "not-an-email", "name": "Nope"},
            "to": [{"email": "ok@example.com"}],
            "subject": "x",
            "text": "y"
        });
        let s = serde_json::to_string(&bad).unwrap();
        let res: Result<Message, _> = serde_json::from_str(&s);
        assert!(res.is_err());
    }
}

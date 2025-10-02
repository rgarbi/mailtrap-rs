use std::iter::Map;
use fake::Optional;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailWithText {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    cc: Option<Vec<EmailAddress>>,
    bcc: Option<Vec<EmailAddress>>,
    reply_to: EmailAddress,
    attachments: Vec<Attachment>,
    headers: Map<String, String>,
    subject: String,
    text: String,
    category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailWithHTML {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    cc: Option<Vec<EmailAddress>>,
    bcc: Option<Vec<EmailAddress>>,
    reply_to: EmailAddress,
    attachments: Vec<Attachment>,
    headers: Map<String, String>,
    subject: String,
    html: String,
    category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailWithTextAndHTML {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    cc: Option<Vec<EmailAddress>>,
    bcc: Option<Vec<EmailAddress>>,
    reply_to: EmailAddress,
    attachments: Vec<Attachment>,
    headers: Map<String, String>,
    subject: String,
    text: String,
    html: String,
    category: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Disposition {
    #[serde(alias = "inline")]
    Inline,
    #[serde(alias = "attachment")]
    Attachment,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Attachment {
    content: String,
    #[serde(alias = "type")]
    content_type: mime,
    filename: String,
    disposition: Disposition,
    content_id: String,
}

impl Attachment {
    pub fn new(content: String, content_type: mime, filename: String, disposition: Disposition, content_id: String) -> Attachment {
        return Attachment {
            content,
            content_type,
            filename,
            disposition,
            content_id,
        };
    }
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailAddress {
    email: ValidEmail,
    name: String,
}

impl EmailAddress {
    pub fn new(email: String, name: String) -> Result<EmailAddress, String> {
        let email_result = ValidEmail::parse(email);

        return match email_result {
            Ok(email) => {
                Ok(EmailAddress {
                    email,
                    name,
                })
            }
            Err(err) => {
                Err(err)
            }
        };
    }
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ValidEmail(String);

impl ValidEmail {
    pub fn parse(s: String) -> Result<ValidEmail, String> {
        if validator::ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
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

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    use super::ValidEmail;

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(ValidEmail::parse(email));
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }
}
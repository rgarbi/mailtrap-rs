use crate::types::email::Body;
use crate::types::email::EmailAddress;
use crate::types::email::{Attachment, Message};
use std::collections::HashMap;

impl Message {
    #[must_use]
    pub fn new(from: EmailAddress, subject: String, body: Body) -> Self {
        Self {
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: None,
            attachments: Vec::new(),
            headers: HashMap::new(),
            custom_variables: HashMap::new(),
            subject,
            body,
            category: None,
        }
    }

    /// Add a CC recipient.
    pub fn to(mut self, addr: EmailAddress) -> Self {
        self.to.push(addr);
        self
    }

    /// Add a CC recipient.
    pub fn cc(mut self, addr: EmailAddress) -> Self {
        self.cc.push(addr);
        self
    }

    /// Add a BCC recipient.
    pub fn bcc(mut self, addr: EmailAddress) -> Self {
        self.bcc.push(addr);
        self
    }

    /// Set a Reply-To address.
    pub fn reply_to(mut self, addr: EmailAddress) -> Self {
        self.reply_to = Some(addr);
        self
    }

    /// Add/replace a custom header. (No CR/LF validation here; add if needed.)
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Add/replace a custom variable.
    pub fn custom_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_variables.insert(key.into(), value.into());
        self
    }

    /// Add an attachment.
    pub fn attachment(mut self, a: Attachment) -> Self {
        self.attachments.push(a);
        self
    }

    /// Set a category.
    pub fn category(mut self, c: impl Into<String>) -> Self {
        self.category = Some(c.into());
        self
    }

    /// Replace the subject (handy when reusing a partially built template).
    pub fn subject(mut self, s: impl Into<String>) -> Self {
        self.subject = s.into();
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::email::{Attachment, Body, Disposition, EmailAddress, ValidMime};
    use claims::{assert_ok, assert_some};
    use std::str::FromStr;

    // -------- helpers --------

    fn addr(email: &str, name: &str) -> EmailAddress {
        assert_ok!(EmailAddress::new(email.to_string(), Some(name.to_string())))
    }

    fn html_attachment() -> Attachment {
        let mime_type = ValidMime::from_str("text/html").unwrap();
        Attachment::new(
            "PGh0bWw+PC9odG1sPg==".to_string(), // "<html></html>" base64
            mime_type,
            "index.html".to_string(),
            Some(Disposition::Attachment),
            Some("cid:index".to_string()),
        )
    }

    // -------- tests --------

    #[test]
    fn new_sets_required_and_defaults() {
        let from = addr("sales@example.com", "Sales");
        let b = Message::new(
            from.clone(),
            "Subject".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        );
        // defaults
        assert!(b.to.is_empty());
        assert!(b.cc.is_empty());
        assert!(b.bcc.is_empty());
        assert!(b.attachments.is_empty());
        assert!(b.headers.is_empty());
        assert!(b.custom_variables.is_empty());
        assert!(b.reply_to.is_none());
        assert!(b.category.is_none());
        assert_eq!(
            b.body,
            Body::Text {
                text: "some text".to_string()
            }
        );
        // required fields preserved
        assert_eq!(b.from.email.to_string(), from.email.to_string());
        assert_eq!(b.subject, "Subject");
    }

    #[test]
    fn text_sets_body() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        );
        match b.body {
            Body::Text { ref text } => assert_eq!(text, "some text"),
            _ => panic!("expected Body::Text"),
        }
    }

    #[test]
    fn html_sets_body() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Html {
                html: "<b>hi</b>".to_string(),
            },
        );
        match b.body {
            Body::Html { ref html } => assert_eq!(html, "<b>hi</b>"),
            _ => panic!("expected Body::Html"),
        }
    }

    #[test]
    fn text_and_html_sets_body() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::TextAndHtml {
                text: "some text".to_string(),
                html: "<b>hi</b>".to_string(),
            },
        );
        match b.body {
            Body::TextAndHtml { ref text, ref html } => {
                assert_eq!(text, "some text");
                assert_eq!(html, "<b>hi</b>");
            }
            _ => panic!("expected Body::TextAndHtml"),
        }
    }

    #[test]
    fn reply_to_sets_option() {
        let reply = addr("reply@example.com", "Reply");
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .reply_to(reply.clone());
        let set = assert_some!(b.reply_to);
        assert_eq!(set.email.to_string(), reply.email.to_string());
    }

    #[test]
    fn header_inserts_and_overwrites() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .header("X-Id", "1")
        .header("X-Trace", "abc")
        // overwrite X-Id
        .header("X-Id", "2");

        assert_eq!(b.headers.len(), 2);
        assert_eq!(b.headers.get("X-Id").unwrap(), "2");
        assert_eq!(b.headers.get("X-Trace").unwrap(), "abc");
    }

    #[test]
    fn custom_var_inserts_and_overwrites() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .custom_var("user_id", "u1")
        .custom_var("batch_id", "b1")
        .custom_var("user_id", "u2"); // overwrite

        assert_eq!(b.custom_variables.len(), 2);
        assert_eq!(b.custom_variables.get("user_id").unwrap(), "u2");
        assert_eq!(b.custom_variables.get("batch_id").unwrap(), "b1");
    }

    #[test]
    fn cc_and_bcc_collect_multiple() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .cc(addr("jane@example.com", "Jane"))
        .cc(addr("mark@example.com", "Mark"))
        .bcc(addr("jim@example.com", "Jim"))
        .bcc(addr("pam@example.com", "Pam"));

        assert_eq!(b.cc.len(), 2);
        assert_eq!(b.bcc.len(), 2);
        assert_eq!(b.cc[0].email.to_string(), "jane@example.com");
        assert_eq!(b.cc[1].email.to_string(), "mark@example.com");
        assert_eq!(b.bcc[0].email.to_string(), "jim@example.com");
        assert_eq!(b.bcc[1].email.to_string(), "pam@example.com");
    }

    #[test]
    fn attachment_is_appended() {
        let a = html_attachment();
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Hi".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .attachment(a);
        assert_eq!(b.attachments.len(), 1);
        assert_eq!(b.attachments[0].filename, "index.html");
        // spot check "type" via mime string
        assert_eq!(b.attachments[0].content_type.as_ref(), "text/html");
    }

    #[test]
    fn category_and_subject_replacement_work() {
        let b = Message::new(
            addr("sales@example.com", "Sales"),
            "Old".to_string(),
            Body::Text {
                text: "some text".to_string(),
            },
        )
        .category("API Test")
        .subject("New Subject");

        assert_eq!(b.category.as_deref(), Some("API Test"));
        assert_eq!(b.subject, "New Subject");
    }

    #[test]
    fn chaining_full_example() {
        let builder = Message::new(
            addr("sales@example.com", "Example Sales Team"),
            "Your Example Order Confirmation".to_string(),
            Body::Text {
                text: "Congratulations on your order no. 1234".to_string(),
            },
        )
        .cc(addr("jane_doe@example.com", "Jane Doe"))
        .bcc(addr("james_doe@example.com", "Jim Doe"))
        .reply_to(addr("reply@example.com", "Reply"))
        .attachment(html_attachment())
        .header("X-Message-Source", "dev.mydomain.com")
        .custom_var("user_id", "45982")
        .custom_var("batch_id", "PSJ-12")
        .category("API Test");

        // body is set
        match builder.body {
            Body::Text { ref text } => assert!(text.contains("order no. 1234")),
            _ => panic!("expected text body"),
        }
        // fields are populated
        assert_eq!(builder.cc.len(), 1);
        assert_eq!(builder.bcc.len(), 1);
        assert!(builder.headers.contains_key("X-Message-Source"));
        assert_eq!(builder.custom_variables.get("batch_id").unwrap(), "PSJ-12");
        assert_eq!(builder.category.as_deref(), Some("API Test"));
    }
}

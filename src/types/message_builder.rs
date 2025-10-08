use crate::types::email::Attachment;
use crate::types::email::Body;
use crate::types::email::EmailAddress;
use std::collections::HashMap;

#[must_use] // nudge users to finish the builder chain
#[derive(Clone, Debug)]
pub struct MessageBuilder {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    cc: Vec<EmailAddress>,
    bcc: Vec<EmailAddress>,
    reply_to: Option<EmailAddress>,
    attachments: Vec<Attachment>,
    headers: HashMap<String, String>,
    custom_variables: HashMap<String, String>,
    subject: String,
    body: Option<Body>,
    category: Option<String>,
}

impl MessageBuilder {
    /// Start a new builder. Requires the *always-required* fields up front.
    pub fn new(from: EmailAddress, subject: impl Into<String>) -> Self {
        Self {
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: None,
            attachments: Vec::new(),
            headers: HashMap::new(),
            custom_variables: HashMap::new(),
            subject: subject.into(),
            body: None,
            category: None,
        }
    }
}

impl MessageBuilder {
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

    pub fn text(mut self, t: impl Into<String>) -> Self {
        self.body = Some(Body::Text { text: t.into() });
        self
    }
    pub fn html(mut self, h: impl Into<String>) -> Self {
        self.body = Some(Body::Html { html: h.into() });
        self
    }
    pub fn text_and_html(mut self, t: impl Into<String>, h: impl Into<String>) -> Self {
        self.body = Some(Body::TextAndHtml {
            text: t.into(),
            html: h.into(),
        });
        self
    }
}

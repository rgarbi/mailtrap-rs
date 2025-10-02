use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
    base_url: url::Url,
}

pub struct ClientBuilder {/* opts: base_url, auth, timeouts … */}

impl Client {}

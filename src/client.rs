use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
    base_url: url::Url,
    transport: Arc<DynTransport>,
}

pub struct ClientBuilder {/* opts: base_url, auth, timeouts … */}

impl Client {
    pub fn builder() -> ClientBuilder { /* … */
    }
    pub fn users(&self) -> resources::users::Users<'_> {
        resources::users::Users { client: self }
    }
    // other resource accessors…
}

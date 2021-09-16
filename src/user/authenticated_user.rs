pub struct AuthenticatedUser {
    pub graphql: String,
    pub client: reqwest::blocking::Client,
    pub user_id: i64,
}

impl AuthenticatedUser {
    pub fn post(&self) -> reqwest::blocking::RequestBuilder {
        self
            .client
            .post(&self.graphql)
    }
}

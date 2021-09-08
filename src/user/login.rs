use clap::{AppSettings, Clap};
use anyhow::{Result, Context};
use graphql_client::{GraphQLQuery, Response};

use crate::user::{APP_USER_AGENT, JwtToken};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/auth.graphql",
    response_derives = "Debug",
)]
pub struct Auth;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UserLogin {
    /// MusicBot GraphQL endpoint
    #[clap(long)]
    pub endpoint: String,

    /// MusicBot user email
    #[clap(long)]
    pub email: String,

    /// MusicBot user password
    #[clap(long)]
    pub password: String,
}

impl UserLogin {
    pub fn new_token(&self) -> Result<String> {
        let auth_variables = auth::Variables {
            email: self.email.clone(),
            password: self.password.clone(),
        };
        let endpoint = &self.endpoint;

        let request_body = Auth::build_query(auth_variables);
        let response_body: Response<auth::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body
            .data.context("missing authentication response data")?
            .authenticate.context("missing authorization response")?
            .jwt_token.context("missing token in response")
    }
}

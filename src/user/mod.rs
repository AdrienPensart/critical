pub mod whoami;
pub mod login;
pub mod accounts;
pub mod register;
pub mod unregister;

use clap::{AppSettings, Clap};
use graphql_client::{GraphQLQuery, Response};
use anyhow::{Result, Context};

use crate::user::login::UserLogin;

type JwtToken = String;
type Datetime = String;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct User {
    /// MusicBot user credentials
    #[clap(flatten)]
    pub user_login: UserLogin,

    /// MusicBot token
    #[clap(short, long)]
    pub token: Option<String>,
}

impl User {
    pub fn authenticate(&self) -> Result<AuthenticatedUser> {
        let token: String = match &self.token {
            Some(token) => token.clone(),
            None => self.user_login.new_token()?
        };
        let endpoint = &self.user_login.endpoint;

        let authorization = format!("Bearer {}", token);
        let authenticated_client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&authorization)
                        .unwrap(),
                ))
                .collect(),
            )
            .build()?;

        let request_body = CurrentUserId::build_query(current_user_id::Variables);
        let response_body: Response<current_user_id::ResponseData> = authenticated_client
            .post(endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        let user_id = response_body
            .data.context("missing user id response data")?
            .current_musicbot.context("missing user id")?;

        Ok(AuthenticatedUser {
            endpoint: endpoint.clone(),
            client: authenticated_client,
            user_id,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/current_musicbot.graphql",
    response_derives = "Debug",
)]
pub struct CurrentUserId;

pub struct AuthenticatedUser {
    pub endpoint: String,
    pub client: reqwest::blocking::Client,
    pub user_id: i64,
}

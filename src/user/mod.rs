pub mod whoami;
pub mod login;
pub mod accounts;
pub mod register;
pub mod unregister;

use clap::{AppSettings, Clap};
use graphql_client::{GraphQLQuery, Response};
use anyhow::{Result, Context, anyhow, bail};

//use crate::err_on_some::ErrOnSome;
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
#[clap(about = "User credentials")]
pub struct User {
    /// MusicBot GraphQL endpoint
    #[clap(long)]
    pub endpoint: String,

    /// MusicBot token
    #[clap(short, long)]
    pub token: Option<String>,

    /// MusicBot user email
    #[clap(short, long, required_unless_present = "token")]
    pub email: Option<String>,

    /// MusicBot user password
    #[clap(short, long, required_unless_present = "token")]
    pub password: Option<String>,
}

impl User {
    pub fn authenticate(&self) -> Result<AuthenticatedUser> {
        let token: String = match &self.token {
            Some(token) => token.clone(),
            None => match (&self.email, &self.password) {
                (Some(email), Some(password)) => {
                    let user_login = UserLogin {
                        endpoint: self.endpoint.clone(),
                        email: email.clone(),
                        password: password.clone(),
                    };
                    user_login.new_token()?
                },
                _ => bail!("You need to specify a token or email/password")
            }
        };
        let endpoint = &self.endpoint;

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

        response_body.errors.map(|errors| Err::<(), _>(anyhow!("{:?}", errors))).transpose()?;

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

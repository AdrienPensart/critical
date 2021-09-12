pub mod queries;
pub mod login;
pub mod accounts;
pub mod register;

use clap::{AppSettings, Clap};
use graphql_client::{GraphQLQuery, Response};
use anyhow::{Result, Context, bail};

use crate::err_on_some::ErrOnSome;
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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/unregister.graphql",
    response_derives = "Debug",
)]
pub struct Unregister;

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
            .post(&self.endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;

        let user_id = response_body
            .data.context("missing user id response data")?
            .current_musicbot.context("missing user id")?;

        Ok(AuthenticatedUser {
            endpoint: self.endpoint.clone(),
            client: authenticated_client,
            user_id,
        })
    }

    pub fn whoami(&self) -> Result<whoami::WhoamiUser> {
        let authenticated_user = self.authenticate()?;
        let variables = whoami::Variables {
            user_id: authenticated_user.user_id,
        };

        let request_body = Whoami::build_query(variables);
        let response = authenticated_user
            .client
            .post(&self.endpoint)
            .json(&request_body)
            .send()?;

        let response_body: Response<whoami::ResponseData> = response.json()?;
        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;

        response_body
            .data.context("missing whoami response data")?
            .user.context("missing user data")
    }

    pub fn unregister(&self) -> Result <()> {
        let authenticated_user = self.authenticate()?;
        let request_body = Unregister::build_query(unregister::Variables);
        let response_body: Response<unregister::ResponseData> = authenticated_user
            .client
            .post(&self.endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;

        response_body
            .data.context("missing unregister response data")?
            .unregister_user.context("missing unregister user response data")?
            .client_mutation_id.context("missing client mutation id in response")?;

        Ok(())
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/whoami.graphql",
    response_derives = "Debug",
)]
pub struct Whoami;

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

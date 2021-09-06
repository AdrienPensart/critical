use anyhow::{Result, Context};
use graphql_client::{GraphQLQuery, Response};
use clap::{AppSettings, Clap};

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

type JwtToken = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/auth.graphql",
    response_derives = "Debug",
)]
pub struct Auth;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/register.graphql",
    response_derives = "Debug",
)]
pub struct Register;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/current_musicbot.graphql",
    response_derives = "Debug",
)]
pub struct CurrentUserId;

pub struct AuthenticatedUser {
    pub endpoint: String,
    pub client: reqwest::blocking::Client,
    pub user_id: i64,
}

#[derive(Clap, Debug)]
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
        let endpoint = self.endpoint;

        let request_body = Auth::build_query(auth_variables);
        let response_body: Response<auth::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(&endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body
            .data.context("missing authentication response data")?
            .authenticate.context("missing authorization response")?
            .jwt_token.context("missing token in response")
    }
}

#[derive(Clap, Debug)]
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
        let endpoint = self.user_login.endpoint;

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
            .post(&endpoint)
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
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "User management")]
pub enum Group {
    Register(UserRegister),
    Unregister(UserUnregister),
    Login(UserLogin),
    List(UserList),
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UserRegister {
    /// MusicBot user
    #[clap(flatten)]
    pub user_login: UserLogin,

    /// MusicBot user first name
    #[clap(long)]
    pub first_name: Option<String>,

    /// MusicBot user last name
    #[clap(long)]
    pub last_name: Option<String>,
}

impl UserRegister {
    fn register(&self) -> Result<User> {
        let register_variables = register::Variables {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: Some(self.user_login.email.clone()),
            password: Some(self.user_login.password.clone()),
        };
        let endpoint = self.user_login.endpoint;

        let request_body = Register::build_query(register_variables);
        let response_body: Response<auth::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(&endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body
            .data.context("missing authentication response data")?
            .authenticate.context("missing authorization response")?
            .jwt_token.context("missing token in response");

        Ok(User {
            user_login: self.user_login,
            token: None,
        })
    }
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UserUnregister {
    /// MusicBot user
    #[clap(flatten)]
    pub user: User,
}

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct UserList {
    /// MusicBot user
    #[clap(flatten)]
    pub user: User,
}

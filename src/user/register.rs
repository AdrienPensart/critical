use clap::{AppSettings, Clap};
use anyhow::{Result, Context};
use graphql_client::{GraphQLQuery, Response};

use crate::user::{User, JwtToken, APP_USER_AGENT};
use crate::user::login::UserLogin;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/register.graphql",
    response_derives = "Debug",
)]
pub struct Register;


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
    pub fn register(&self) -> Result<User> {
        let variables = register::Variables {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.user_login.email.clone(),
            password: self.user_login.password.clone(),
        };
        let endpoint = &self.user_login.endpoint;

        let request_body = Register::build_query(variables);
        let response_body: Response<register::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        println!("{:?}", response_body);

        let token = response_body
            .data.context("missing register user response data")?
            .register_user.context("missing register user response")?
            .jwt_token.context("missing client mutation id in response")?;

        Ok(User {
            user_login: self.user_login.clone(),
            token: Some(token),
        })
    }
}

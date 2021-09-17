use clap::{AppSettings, Clap};
use anyhow::{Result, Context, bail};
use graphql_client::{GraphQLQuery, Response};

use crate::err_on_some::ErrOnSome;
use crate::user::{User, JwtToken, APP_USER_AGENT};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/register.graphql",
    response_derives = "Debug",
)]
pub struct Register;


#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Register a new user")]
pub struct UserRegister {
    /// MusicBot GraphQL endpoint
    #[clap(short, long, visible_alias = "endpoint")]
    pub graphql: String,

    /// MusicBot user email
    #[clap(long)]
    pub email: String,

    /// MusicBot user password
    #[clap(long)]
    pub password: String,

    /// MusicBot user first name
    #[clap(long, default_value = "")]
    pub first_name: String,

    /// MusicBot user last name
    #[clap(long, default_value = "")]
    pub last_name: String,
}

impl UserRegister {
    pub fn register(&self) -> Result<User> {
        let variables = register::Variables {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        };

        let request_body = Register::build_query(variables);
        let response_body: Response<register::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(&self.graphql)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        let token = response_body
            .data.with_context(|| format!("missing register user response data : {:?}", response_copy))?
            .register_user.with_context(|| format!("missing register user response : {:?}", response_copy))?
            .jwt_token.with_context(|| format!("missing token in response : {:?}", response_copy))?;

        Ok(User {
            graphql: self.graphql.clone(),
            token: Some(token),
            email: Some(self.email.clone()),
            password: Some(self.password.clone()),
        })
    }
}

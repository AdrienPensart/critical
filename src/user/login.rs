use clap::Parser;
use anyhow::{Result, Context, bail};
use graphql_client::{GraphQLQuery, Response};

use crate::err_on_some::ErrOnSome;
use crate::user::{APP_USER_AGENT, JwtToken};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/auth.graphql",
    response_derives = "Debug",
)]
pub struct Auth;

#[derive(Parser, Debug, Clone)]
#[clap(about = "Get a new token")]
pub struct UserLogin {
    /// MusicBot GraphQL endpoint
    #[clap(long, short, visible_alias = "endpoint")]
    pub graphql: String,

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

        let request_body = Auth::build_query(auth_variables);
        let response_body: Response<auth::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(&self.graphql)
            .json(&request_body)
            .send()?
            .error_for_status()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing authentication response data : {:?}", response_copy))?
            .authenticate.with_context(|| format!("missing authorization response : {:?}", response_copy))?
            .jwt_token.with_context(|| format!("missing token in response : {:?}", response_copy))
    }
}

use clap::{AppSettings, Clap};
use anyhow::{Result, Context, anyhow};
use graphql_client::{GraphQLQuery, Response};

use crate::user::{User, APP_USER_AGENT};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/unregister.graphql",
    response_derives = "Debug",
)]
pub struct Unregister;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Unregister a user")]
pub struct UserUnregister {
    /// MusicBot user
    #[clap(flatten)]
    pub user: User,
}

impl UserUnregister {
    pub fn unregister(&self) -> Result <()> {
        let request_body = Unregister::build_query(unregister::Variables);
        let endpoint = &self.user.endpoint;
        let response_body: Response<unregister::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.map(|errors| Err::<(), _>(anyhow!("{:?}", errors))).transpose()?;

        response_body
            .data.context("missing unregister response data")?
            .unregister_user.context("missing unregister user response data")?
            .client_mutation_id.context("missing client mutation id in response")?;

        Ok(())
    }
}

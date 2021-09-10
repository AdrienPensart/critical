use clap::{AppSettings, Clap};
use anyhow::{Result, Context, anyhow};
use graphql_client::{GraphQLQuery, Response};

use crate::user::{User, APP_USER_AGENT, Datetime};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/whoami.graphql",
    response_derives = "Debug",
)]
pub struct Whoami;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Print information about me")]
pub struct UserWhoami {
    /// MusicBot user
    #[clap(flatten)]
    pub user: User,
}

impl UserWhoami {
    pub fn whoami(&self) -> Result<whoami::WhoamiUser> {
        let authenticated_user = self.user.authenticate()?;
        let variables = whoami::Variables {
            user_id: authenticated_user.user_id,
        };

        let request_body = Whoami::build_query(variables);
        let endpoint = &self.user.endpoint;
        let response_body: Response<whoami::ResponseData> = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?
            .post(endpoint)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.map(|errors| Err::<(), _>(anyhow!("{:?}", errors))).transpose()?;

        Ok(response_body
            .data.context("missing whoami response data")?
            .user.context("missing user data")?
        )
    }
}

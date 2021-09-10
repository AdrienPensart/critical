use clap::{AppSettings, Clap};
use anyhow::{Result, Context, anyhow};
use graphql_client::{GraphQLQuery, Response};

use crate::user::{APP_USER_AGENT, Datetime};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot_admin.json",
    query_path = "src/user/queries/user_account_list.graphql",
    response_derives = "Debug",
)]
pub struct UserAccountList;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "List all users")]
pub struct AdminListUsers {
    /// MusicBot GraphQL endpoint
    #[clap(long)]
    pub endpoint: String,

    /// MusicBot admin user
    #[clap(long)]
    pub user: String,

    /// MusicBot admin password
    #[clap(long)]
    pub password: String,
}

impl AdminListUsers {
    pub fn users(&self) -> Result<Vec<user_account_list::UserAccountListUserAccountsList>> {
        let endpoint = &self.endpoint;

        let authenticated_client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let request_body = UserAccountList::build_query(user_account_list::Variables);
        let response_body: Response<user_account_list::ResponseData> = authenticated_client
            .post(endpoint)
            .basic_auth(&self.user, Some(&self.password))
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.map(|errors| Err::<(), _>(anyhow!("{:?}", errors))).transpose()?;

        Ok(response_body
            .data.context("missing users response data")?
            .user_accounts_list.context("missing user id")?
        )
    }
}

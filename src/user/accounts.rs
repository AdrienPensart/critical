use clap::Parser;
use anyhow::{Result, Context, bail};
use graphql_client::{GraphQLQuery, Response};

use crate::err_on_some::ErrOnSome;
use crate::user::{APP_USER_AGENT, Datetime};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot_admin.json",
    query_path = "src/user/queries/user_account_list.graphql",
    response_derives = "Debug",
)]
pub struct UserAccountList;

#[derive(Parser, Debug)]
#[clap(about = "List all users")]
pub struct AdminListUsers {
    /// MusicBot GraphQL endpoint
    #[clap(long, short, visible_alias = "endpoint")]
    pub graphql: String,
}

impl AdminListUsers {
    pub fn users(&self) -> Result<Vec<user_account_list::UserAccountListUserAccountsList>> {
        let authenticated_client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let request_body = UserAccountList::build_query(user_account_list::Variables);
        let response_body: Response<user_account_list::ResponseData> = authenticated_client
            .post(&self.graphql)
            .json(&request_body)
            .send()?
            .error_for_status()?
            .json()?;

        println!("plop");

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing users response data : {:?}", response_copy))?
            .user_accounts_list.with_context(|| format!("missing user id : {:?}", response_copy))
    }
}

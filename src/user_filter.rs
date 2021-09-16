use anyhow::{Result, Context, bail};
use clap::{AppSettings, Clap};
use graphql_client::{GraphQLQuery, Response};

use crate::err_on_some::ErrOnSome;
use crate::user::User;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Music filter")]
pub struct UserFilter {
    #[clap(flatten)]
    user: User,

    filter: String,
}

pub fn search_filter(user: &User, name: String) -> Result<get_filter::GetFilterFiltersList> {
    let authenticated_user = user.authenticate()?;
    let variables = get_filter::Variables { name: name.clone() };
    let request_body = GetFilter::build_query(variables);
    let response = authenticated_user
        .post()
        .json(&request_body)
        .send()?;

    let response_body: Response<get_filter::ResponseData> = response.json()?;
    response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;

    let response_copy = format!("{:?}", response_body.data);
    response_body
        .data.with_context(|| format!("missing user id response data : {}", response_copy))?
        .filters_list.with_context(|| format!("missing filters list : {}", response_copy))?
        .into_iter()
        .next()
        .with_context(|| format!("filter '{}' not found", name))
}

impl UserFilter {
    pub fn get(self) -> Result<get_filter::GetFilterFiltersList> {
        search_filter(&self.user, self.filter)
    }

    pub fn delete(self) -> Result<()> {
        let authenticated_user = self.user.authenticate()?;
        let variables = delete_filter::Variables { name: self.filter };
        let request_body = DeleteFilter::build_query(variables);
        let response = authenticated_user
            .post()
            .json(&request_body)
            .send()?;

        let response_body: Response<delete_filter::ResponseData> = response.json()?;
        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing delete filter response data : {}", response_copy))?
            .delete_filter.with_context(|| format!("missing delete filter response data : {}", response_copy))?
            .client_mutation_id.with_context(|| format!("missing client mutation id in response : {}", response_copy))?;

        Ok(())
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/filter/queries/get.graphql",
    response_derives = "Debug",
)]
pub struct GetFilter;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/filter/queries/delete.graphql",
    response_derives = "Debug",
)]
pub struct DeleteFilter;

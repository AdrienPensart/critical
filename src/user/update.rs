use anyhow::{Result, Context, bail};
use clap::Parser;
use graphql_client::{GraphQLQuery, Response};

use crate::err_on_some::ErrOnSome;
use crate::user::User;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/update.graphql",
    response_derives = "Debug",
)]
pub struct Update;


#[derive(Parser, Debug)]
#[clap(about = "Update user")]
pub struct UserUpdate {
    #[clap(flatten)]
    user: User,

    /// MusicBot user first name
    #[clap(long)]
    pub new_first_name: Option<String>,

    /// MusicBot user last name
    #[clap(long)]
    pub new_last_name: Option<String>,
}

impl UserUpdate {
    pub fn update(&self) -> Result<()> {
        let authenticated_user = self.user.authenticate()?;
        let variables = update::Variables {
            user_id: authenticated_user.user_id,
            first_name: self.new_first_name.clone(),
            last_name: self.new_last_name.clone()
        };

        let request_body = Update::build_query(variables);
        let response_body: Response<update::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .error_for_status()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        let _client_mutation_id = response_body
            .data.with_context(|| format!("missing update response data : {}", response_copy))?
            .update_user.with_context(|| format!("missing update user response data : {}", response_copy))?
            .client_mutation_id;

        Ok(())
    }
}

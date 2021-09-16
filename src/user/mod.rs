pub mod authenticated_user;
pub mod queries;
pub mod login;
pub mod accounts;
pub mod register;

use clap::{AppSettings, Clap};
use graphql_client::{GraphQLQuery, Response};
use anyhow::{Result, Context, bail};

use crate::types::{BigInt, JwtToken, Datetime};
use crate::err_on_some::ErrOnSome;
use crate::user::login::UserLogin;
use crate::user::authenticated_user::AuthenticatedUser;
use crate::filter::{upsert_filter, DEFAULT_FILTERS};

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "User credentials")]
pub struct User {
    /// MusicBot GraphQL endpoint
    #[clap(short, long, visible_alias = "endpoint")]
    pub graphql: String,

    /// MusicBot token
    #[clap(short, long)]
    pub token: Option<String>,

    /// MusicBot user email
    #[clap(short, long, required_unless_present = "token")]
    pub email: Option<String>,

    /// MusicBot user password
    #[clap(short, long, required_unless_present = "token")]
    pub password: Option<String>,
}

impl User {
    pub fn authenticate(&self) -> Result<AuthenticatedUser> {
        let token: String = match &self.token {
            Some(token) => token.clone(),
            None => match (&self.email, &self.password) {
                (Some(email), Some(password)) => {
                    let user_login = UserLogin {
                        graphql: self.graphql.clone(),
                        email: email.clone(),
                        password: password.clone(),
                    };
                    user_login.new_token()?
                },
                _ => bail!("You need to specify a token or email/password")
            }
        };
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
            .post(&self.graphql)
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        let user_id = response_body
            .data.with_context(|| format!("missing user id response data : {}", response_copy))?
            .current_musicbot.with_context(|| format!("missing user id : {}", response_copy))?;

        Ok(AuthenticatedUser {
            graphql: self.graphql.clone(),
            client: authenticated_client,
            user_id,
        })
    }

    pub fn whoami(&self) -> Result<whoami::WhoamiUser> {
        let authenticated_user = self.authenticate()?;
        let variables = whoami::Variables {
            user_id: authenticated_user.user_id,
        };

        let request_body = Whoami::build_query(variables);
        let response_body: Response<whoami::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing whoami response data : {}", response_copy))?
            .user.with_context(|| format!("missing user data : {}", response_copy))
    }

    pub fn unregister(&self) -> Result<()> {
        let authenticated_user = self.authenticate()?;
        let request_body = Unregister::build_query(unregister::Variables);
        let response_body: Response<unregister::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing unregister response data : {}", response_copy))?
            .unregister_user.with_context(|| format!("missing unregister user response data : {}", response_copy))?
            .client_mutation_id.with_context(|| format!("missing client mutation id in response : {}", response_copy))?;

        Ok(())
    }

    pub fn clean_musics(&self) -> Result<i64> {
        let authenticated_user = self.authenticate()?;
        let request_body = CleanMusics::build_query(clean_musics::Variables);
        let response_body: Response<clean_musics::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing clean musics response : {}", response_copy))?
            .delete_all_music.with_context(|| format!("missing clean musics response data : {}", response_copy))?
            .big_int.with_context(|| format!("missing client mutation id in response : {}", response_copy))?
            .parse::<i64>().with_context(|| format!("cannot get deleted musics : {}", response_copy))
    }

    pub fn count_filters(self) -> Result<i64> {
        let authenticated_user = self.authenticate()?;
        let request_body = CountFilters::build_query(count_filters::Variables);
        let response_body: Response<count_filters::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        Ok(response_body
            .data.with_context(|| format!("missing filter count response data : {}", response_copy))?
            .filters.with_context(|| format!("missing filter count data : {}", response_copy))?
            .total_count)
    }

    pub fn load_default_filters(self) -> Result<()> {
        let authenticated_user = self.authenticate()?;

        for default_filter in DEFAULT_FILTERS.iter() {
            let request_body = default_filter.create_upsert_query(authenticated_user.user_id);
            let response_body: Response<upsert_filter::ResponseData> = authenticated_user
                .post()
                .json(&request_body)
                .send()?
                .json()?;

            response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
            let response_copy = format!("{:?}", response_body.data);

            let _client_mutation_id = response_body
                .data.with_context(|| format!("missing filter upsert response data : {}", response_copy))?
                .upsert_filter.with_context(|| format!("missing filter upsert data : {}", response_copy))?
                .client_mutation_id;
        }
        Ok(())
    }

    pub fn list_filters(self) -> Result<Vec<list_filters::ListFiltersFiltersList>> {
        let authenticated_user = self.authenticate()?;
        let request_body = ListFilters::build_query(list_filters::Variables);
        let response_body: Response<list_filters::ResponseData> = authenticated_user
            .post()
            .json(&request_body)
            .send()?
            .json()?;

        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        let filters_list = response_body
            .data.with_context(|| format!("missing filter list response data : {}", response_copy))?
            .filters_list;

        match filters_list {
            None => bail!("missing filters list data : {}", response_copy),
            Some(filters_list) => Ok(filters_list)
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/whoami.graphql",
    response_derives = "Debug",
)]
pub struct Whoami;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/current_musicbot.graphql",
    response_derives = "Debug",
)]
pub struct CurrentUserId;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/user/queries/unregister.graphql",
    response_derives = "Debug",
)]
pub struct Unregister;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/music/queries/clean.graphql",
    response_derives = "Debug",
)]
pub struct CleanMusics;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/filter/queries/count.graphql",
    response_derives = "Debug",
)]
pub struct CountFilters;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/musicbot.json",
    query_path = "src/filter/queries/list.graphql",
    response_derives = "Debug",
)]
pub struct ListFilters;

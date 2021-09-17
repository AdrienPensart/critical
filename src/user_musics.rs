use anyhow::{Result, Context, bail};
use clap::{AppSettings, Clap};
use graphql_client::Response;

use crate::helpers::vec_option_to_vec;
use crate::err_on_some::ErrOnSome;
use crate::user::User;
use crate::user_filter::search_filter;
use crate::filter::{playlist, stats, Filter};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Music filter")]
pub struct UserMusics {
    #[clap(flatten)]
    user: User,

    #[clap(flatten)]
    filter: Filter,
}

impl UserMusics {
    pub fn get_filter(self) -> Result<Filter> {
        let filter = if !self.filter.name.is_empty() {
            let filter = search_filter(&self.user, self.filter.name.clone())?;
            Filter {
                name: filter.name,
                limit: filter.limit as i32,
                shuffle: filter.shuffle,
                min_duration: filter.min_duration as i32,
                max_duration: filter.max_duration as i32,
                titles: vec_option_to_vec(filter.titles),
                no_titles: vec_option_to_vec(filter.no_titles),
                artists: vec_option_to_vec(filter.artists),
                no_artists: vec_option_to_vec(filter.no_artists),
                albums: vec_option_to_vec(filter.albums),
                no_albums: vec_option_to_vec(filter.no_albums),
                genres: vec_option_to_vec(filter.genres),
                no_genres: vec_option_to_vec(filter.no_genres),
                keywords: vec_option_to_vec(filter.keywords),
                no_keywords: vec_option_to_vec(filter.no_keywords),
                min_rating: filter.min_rating,
                max_rating: filter.max_rating,
            }
        } else {
            self.filter
        };
        Ok(filter)
    }

    pub fn stats(self) -> Result<stats::StatsDoStat> {
        let authenticated_user = self.user.authenticate()?;
        let filter = self.get_filter()?;

        let request_body = filter.create_stats_query();
        let response = authenticated_user
            .post()
            .json(&request_body)
            .send()?;

        let response_body: Response<stats::ResponseData> = response.json()?;
        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing stats response data : {}", response_copy))?
            .do_stat.with_context(|| format!("missing stats : {}\n{}\n{}", response_copy, serde_json::to_string_pretty(&request_body.variables).unwrap(), request_body.query))
    }

    pub fn playlist(self) -> Result<Vec<playlist::PlaylistPlaylistList>> {
        let authenticated_user = self.user.authenticate()?;
        let filter = self.get_filter()?;

        let request_body = filter.create_playlist_query();
        let response = authenticated_user
            .post()
            .json(&request_body)
            .send()?;

        let response_body: Response<playlist::ResponseData> = response.json()?;
        response_body.errors.err_on_some(|| bail!("{:?}", response_body.errors))?;
        let response_copy = format!("{:?}", response_body.data);

        response_body
            .data.with_context(|| format!("missing playlist response data : {}", response_copy))?
            .playlist_list.with_context(|| format!("missing playlist : {}\n{}\n{}\n", response_copy, serde_json::to_string_pretty(&request_body.variables).unwrap(), request_body.query))
    }
}

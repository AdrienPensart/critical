use anyhow::Result;
use clap::{AppSettings, Clap};
use prettytable::Table;

use crate::helpers::vec_option_to_vec;
use crate::group_dispatch::GroupDispatch;
use crate::user_filter::UserFilter;
use crate::user::User;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Filter management")]
pub enum Group {
    Count(User),
    Load(User),
    List(User),
    Show(UserFilter),
    Delete(UserFilter),
}

impl GroupDispatch for Group {
    fn dispatch(self) -> Result<()> {
        match self {
            Group::Count(user) => {
                let count = user.count_filters()?;
                println!("Filter count : {}", count);
                Ok(())
            },
            Group::Load(user) => {
                user.load_default_filters()
            }
            Group::List(user) => {
                let filters = user.list_filters()?;
                let mut table = Table::new();
                table.add_row(row!["Name", "Duration", "Rating", "Artists", "Albums", "Genres", "Titles", "Keywords", "Shuffle", "Limit"]);
                for filter in filters {
                    let min_duration = filter.min_duration;
                    let max_duration = filter.max_duration;

                    let min_rating = filter.min_rating;
                    let max_rating = filter.max_rating;

                    let artists = vec_option_to_vec(filter.artists);
                    let no_artists = vec_option_to_vec(filter.no_artists);

                    let albums = vec_option_to_vec(filter.albums);
                    let no_albums = vec_option_to_vec(filter.no_albums);

                    let genres = vec_option_to_vec(filter.genres);
                    let no_genres = vec_option_to_vec(filter.no_genres);

                    let titles = vec_option_to_vec(filter.titles);
                    let no_titles = vec_option_to_vec(filter.no_titles);

                    let keywords = vec_option_to_vec(filter.keywords);
                    let no_keywords = vec_option_to_vec(filter.no_keywords);

                    table.add_row(row![
                        filter.name,
                        format!("min: {}\nmax: {}", min_duration, max_duration),
                        format!("min: {}\nmax: {}", min_rating, max_rating),
                        format!("in: {:?}\nout: {:?}", artists, no_artists),
                        format!("in: {:?}\nout: {:?}", albums, no_albums),
                        format!("in: {:?}\nout: {:?}", genres, no_genres),
                        format!("in: {:?}\nout: {:?}", titles, no_titles),
                        format!("in: {:?}\nout: {:?}", keywords, no_keywords),
                        filter.shuffle,
                        filter.limit,
                    ]);
                }
                table.printstd();
                Ok(())
            }
            Group::Show(user_filter) => {
                println!("{:?}", user_filter.get()?);
                Ok(())
            }
            Group::Delete(user_filter) => {
                user_filter.delete()
            }
        }
    }
}

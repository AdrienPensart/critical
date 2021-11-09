use anyhow::Result;
use clap::Parser;
use prettytable::Table;

use crate::helpers::vec_option_to_vec;
use crate::group_dispatch::GroupDispatch;
use crate::user_filter::UserFilter;
use crate::user::User;

#[derive(Parser, Debug)]
#[clap(about = "Filter management")]
pub enum Group {
    Count(User),
    Load(User),
    List(User),
    Show(UserFilter),
    Delete(UserFilter),
}

fn gen_in_and_out(with: Vec<Option<String>>, without: Vec<Option<String>>) -> String {
    let mut filter = String::new();
    if !with.is_empty() {
        let with_vec = vec_option_to_vec(with);
        filter.push_str(&format!("in: {:?}", with_vec));
    }
    if !without.is_empty() {
        let without_vec = vec_option_to_vec(without);
        if filter.is_empty(){
            filter.push_str(&format!("out: {:?}", without_vec));
        } else {
            filter.push_str(&format!("\nout: {:?}", without_vec));
        }
    }
    filter
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
                    table.add_row(row![
                        filter.name,
                        format!("min: {}\nmax: {}", filter.min_duration, filter.max_duration),
                        format!("min: {}\nmax: {}", filter.min_rating, filter.max_rating),
                        gen_in_and_out(filter.artists, filter.no_artists),
                        gen_in_and_out(filter.albums, filter.no_albums),
                        gen_in_and_out(filter.genres, filter.no_genres),
                        gen_in_and_out(filter.titles, filter.no_titles),
                        gen_in_and_out(filter.keywords, filter.no_keywords),
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

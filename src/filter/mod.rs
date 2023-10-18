use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_FILTERS: [Filter; 11] = [
        Filter { name: "default".to_string(), ..Filter::default() },
        Filter { name: "no artist set".to_string(), artists: vec!["".to_string()], ..Filter::default() },
        Filter { name: "no album set".to_string(), albums: vec!["".to_string()], ..Filter::default() },
        Filter { name: "no title set".to_string(), titles: vec!["".to_string()], ..Filter::default() },
        Filter { name: "no genre set".to_string(), genres: vec!["".to_string()], ..Filter::default() },
        Filter { name: "no rating".to_string(), min_rating: 0.0, max_rating: 0.0, ..Filter::default() },
        Filter { name: "best (4.0+)".to_string(), min_rating: 4.0, no_keywords: vec!["cutoff".to_string(), "bad".to_string(), "demo".to_string(), "intro".to_string()], ..Filter::default() },
        Filter { name: "best (4.5+)".to_string(), min_rating: 4.5, no_keywords: vec!["cutoff".to_string(), "bad".to_string(), "demo".to_string(), "intro".to_string()], ..Filter::default() },
        Filter { name: "best (5.0+)".to_string(), min_rating: 5.0, no_keywords: vec!["cutoff".to_string(), "bad".to_string(), "demo".to_string(), "intro".to_string()], ..Filter::default() },
        Filter { name: "no live".to_string(), no_keywords: vec!["live".to_string()], ..Filter::default() },
        Filter { name: "only live".to_string(), keywords: vec!["live".to_string()], ..Filter::default() },
    ];
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            name: "".to_owned(),
            shuffle: false,
            min_duration: 0,
            max_duration: i32::MAX,
            min_rating: 0.0,
            max_rating: 5.0,
            limit: i32::MAX,
            genres: vec![],
            no_genres: vec![],
            artists: vec![],
            no_artists: vec![],
            albums: vec![],
            no_albums: vec![],
            titles: vec![],
            no_titles: vec![],
            keywords: vec![],
            no_keywords: vec![],
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about = "Music filter")]
pub struct Filter {
    #[clap(long, default_value = "")]
    pub name: String,

    #[clap(long)]
    pub shuffle: bool,

    #[clap(long, default_value = "0")]
    pub min_duration: i32,

    #[clap(long, default_value_t = i32::MAX)]
    pub max_duration: i32,

    #[clap(long, default_value = "0.0")]
    pub min_rating: f64,

    #[clap(long, default_value = "5.0")]
    pub max_rating: f64,

    #[clap(long, default_value_t = i32::MAX)]
    pub limit: i32,

    #[clap(long)]
    pub genres: Vec<String>,

    #[clap(long)]
    pub no_genres: Vec<String>,

    #[clap(long)]
    pub keywords: Vec<String>,

    #[clap(long)]
    pub no_keywords: Vec<String>,

    #[clap(long)]
    pub artists: Vec<String>,

    #[clap(long)]
    pub no_artists: Vec<String>,

    #[clap(long)]
    pub titles: Vec<String>,

    #[clap(long)]
    pub no_titles: Vec<String>,

    #[clap(long)]
    pub albums: Vec<String>,

    #[clap(long)]
    pub no_albums: Vec<String>,
}

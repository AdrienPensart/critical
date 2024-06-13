use crate::music::RATINGS;

const MATCH_ALL: &str = "(.*?)";
const DEFAULT_PATTERN: &str = "";

#[derive(clap::Parser, Debug, serde::Serialize, Default)]
pub struct Filter {
    #[clap(long, default_value_t = 0)]
    pub min_length: i64,
    #[clap(long, default_value_t = i64::MAX)]
    pub max_length: i64,
    #[clap(long, default_value_t = 0)]
    pub min_size: i64,
    #[clap(long, default_value_t = i64::MAX)]
    pub max_size: i64,
    #[clap(long, default_value_t = 0.0, value_parser = validate_rating)]
    pub min_rating: f64,
    #[clap(long, default_value_t = 5.0, value_parser = validate_rating)]
    pub max_rating: f64,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    pub artist: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    pub album: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    pub genre: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    pub title: String,
    #[clap(long, default_value_t = MATCH_ALL.to_string())]
    pub keyword: String,
    #[clap(long, default_value_t = DEFAULT_PATTERN.to_string())]
    pub pattern: String,
    #[clap(long, default_value_t = i64::MAX)]
    pub limit: i64,
}

fn validate_rating(rating_str: &str) -> Result<f64, String> {
    if let Ok(rating) = rating_str.parse::<f64>() {
        if RATINGS.contains(&rating) {
            return Ok(rating);
        }
    };
    Err(format!(
        "{rating_str} is invalid rating, valid values: {}",
        RATINGS
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_FILTERS: std::collections::HashMap<String, Filter> = {
        let mut filters = std::collections::HashMap::new();

        filters.insert("no-artist".to_string(), Filter {
            artist: "^$".to_string(),
            ..Filter::default()
        });

        filters.insert("no-album".to_string(), Filter {
            album: "^$".to_string(),
            ..Filter::default()
        });

        filters.insert("no-title".to_string(), Filter {
            title: "^$".to_string(),
            ..Filter::default()
        });

        filters.insert("no-genre".to_string(), Filter {
            genre: "^$".to_string(),
            ..Filter::default()
        });

        filters.insert("no-rating".to_string(), Filter {
            min_rating: 0.0,
            max_rating: 0.0,
            ..Filter::default()
        });

        filters.insert("best-4.0".to_string(), Filter {
            min_rating: 4.0,
            ..Filter::default()
        });

        filters.insert("best-4.5".to_string(), Filter {
            min_rating: 4.5,
            ..Filter::default()

        });

        filters.insert("best-5.0".to_string(), Filter {
            min_rating: 5.0,
            ..Filter::default()
        });

        filters
    };
}

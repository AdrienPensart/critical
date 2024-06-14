use crate::music::RATINGS;

const fn default_min_length() -> i64 {
    0
}
const fn default_max_length() -> i64 {
    i64::MAX
}

const fn default_min_size() -> i64 {
    0
}
const fn default_max_size() -> i64 {
    i64::MAX
}

const fn default_min_rating() -> f64 {
    0.0
}
const fn default_max_rating() -> f64 {
    5.0
}

const fn default_limit() -> i64 {
    i64::MAX
}

const MATCH_ALL: &str = "(.*?)";
const DEFAULT_PATTERN: &str = "";

fn default_match_all() -> String {
    MATCH_ALL.to_string()
}

fn default_pattern() -> String {
    DEFAULT_PATTERN.to_string()
}

const NO_KEYWORD: &str = "^((?!cutoff|bad|demo|intro).)$";

#[derive(clap::Parser, Debug, serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct Filter {
    #[serde(default = "default_min_length")]
    #[clap(long, default_value_t = default_min_length())]
    pub min_length: i64,

    #[serde(default = "default_max_length")]
    #[clap(long, default_value_t = default_max_length())]
    pub max_length: i64,

    #[serde(default = "default_min_size")]
    #[clap(long, default_value_t = default_min_size())]
    pub min_size: i64,

    #[serde(default = "default_max_size")]
    #[clap(long, default_value_t = default_max_size())]
    pub max_size: i64,

    #[serde(default = "default_min_rating")]
    #[clap(long, default_value_t = default_min_rating(), value_parser = validate_rating)]
    pub min_rating: f64,

    #[serde(default = "default_max_rating")]
    #[clap(long, default_value_t = default_max_rating(), value_parser = validate_rating)]
    pub max_rating: f64,

    #[serde(default = "default_match_all")]
    #[clap(long, default_value_t = default_match_all())]
    pub artist: String,

    #[serde(default = "default_match_all")]
    #[clap(long, default_value_t = default_match_all())]
    pub album: String,

    #[serde(default = "default_match_all")]
    #[clap(long, default_value_t = default_match_all())]
    pub genre: String,

    #[serde(default = "default_match_all")]
    #[clap(long, default_value_t = default_match_all())]
    pub title: String,

    #[serde(default = "default_match_all")]
    #[clap(long, default_value_t = default_match_all())]
    pub keyword: String,

    #[serde(default = "default_pattern")]
    #[clap(long, default_value_t = default_pattern())]
    pub pattern: String,

    #[serde(default = "default_limit")]
    #[clap(long, default_value_t = default_limit())]
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
            keyword: NO_KEYWORD.to_string(),
            ..Filter::default()
        });

        filters.insert("best-4.5".to_string(), Filter {
            min_rating: 4.5,
            keyword: NO_KEYWORD.to_string(),
            ..Filter::default()

        });

        filters.insert("best-5.0".to_string(), Filter {
            min_rating: 5.0,
            keyword: NO_KEYWORD.to_string(),
            ..Filter::default()
        });

        filters
    };
}

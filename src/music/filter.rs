use super::errors::CriticalErrorKind;
use super::ratings::RATINGS;

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
const EMPTY_STRING_REGEX: &str = "^$";
const DEFAULT_PATTERN: &str = "";
const NO_KEYWORD: &str = "^((?!cutoff|bad|demo|intro).)$";

fn default_match_all() -> String {
    MATCH_ALL.to_string()
}

fn default_pattern() -> String {
    DEFAULT_PATTERN.to_string()
}

#[derive(clap::Parser, Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
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

#[derive(clap::Parser, Default, Clone)]
pub struct Filters {
    #[clap(flatten)]
    filter: Filter,

    #[clap(name = "filter", long, value_parser = validate_filters)]
    filters: Vec<Filter>,
}

impl Filters {
    #[must_use]
    pub fn all(&self) -> Vec<Filter> {
        let mut filters = self.filters.clone();
        if filters.is_empty() || self.filter != Filter::default() {
            filters.push(self.filter.clone());
        }
        filters
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            min_length: default_min_length(),
            max_length: default_max_length(),
            min_size: default_min_size(),
            max_size: default_max_size(),
            min_rating: default_min_rating(),
            max_rating: default_max_rating(),
            artist: default_match_all(),
            album: default_match_all(),
            genre: default_match_all(),
            title: default_match_all(),
            keyword: default_match_all(),
            pattern: default_pattern(),
            limit: default_limit(),
        }
    }
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

pub fn validate_filters(filter: &str) -> Result<Filter, String> {
    match serde_keyvalue::from_key_values::<Filter>(filter) {
        Ok(filter) => {
            if filter.min_rating > filter.max_rating {
                return Err(CriticalErrorKind::InvalidMinMaxRating {
                    min_rating: filter.min_rating,
                    max_rating: filter.max_rating,
                }
                .to_string());
            }
            if filter.min_length > filter.max_length {
                return Err(CriticalErrorKind::InvalidMinMaxLength {
                    min_length: filter.min_length,
                    max_length: filter.max_length,
                }
                .to_string());
            }
            if filter.min_size > filter.max_size {
                return Err(CriticalErrorKind::InvalidMinMaxSize {
                    min_size: filter.min_size,
                    max_size: filter.max_size,
                }
                .to_string());
            }
            Ok(filter)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub static DEFAULT_FILTERS: std::sync::LazyLock<std::collections::HashMap<String, Filter>> =
    std::sync::LazyLock::new(|| {
        let mut filters = std::collections::HashMap::new();

        filters.insert(
            "no-artist".to_string(),
            Filter {
                artist: EMPTY_STRING_REGEX.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "no-album".to_string(),
            Filter {
                album: EMPTY_STRING_REGEX.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "no-title".to_string(),
            Filter {
                title: EMPTY_STRING_REGEX.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "no-genre".to_string(),
            Filter {
                genre: EMPTY_STRING_REGEX.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "no-rating".to_string(),
            Filter {
                min_rating: 0.0,
                max_rating: 0.0,
                ..Filter::default()
            },
        );

        filters.insert(
            "best-4.0".to_string(),
            Filter {
                min_rating: 4.0,
                keyword: NO_KEYWORD.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "best-4.5".to_string(),
            Filter {
                min_rating: 4.5,
                keyword: NO_KEYWORD.to_string(),
                ..Filter::default()
            },
        );

        filters.insert(
            "best-5.0".to_string(),
            Filter {
                min_rating: 5.0,
                keyword: NO_KEYWORD.to_string(),
                ..Filter::default()
            },
        );

        filters
    });

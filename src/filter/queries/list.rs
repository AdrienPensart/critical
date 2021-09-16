pub struct ListFilters;
pub mod list_filters {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ListFilters";
    pub const QUERY : & str = "query ListFilters\n{\n    filtersList {\n        name\n        minDuration\n        maxDuration\n        minRating\n        maxRating\n        artists\n        noArtists\n        albums\n        noAlbums\n        titles\n        noTitles\n        genres\n        noGenres\n        keywords\n        noKeywords\n        shuffle\n        limit\n    }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, Debug)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "filtersList")]
        pub filters_list: Option<Vec<ListFiltersFiltersList>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ListFiltersFiltersList {
        pub name: String,
        #[serde(rename = "minDuration")]
        pub min_duration: Int,
        #[serde(rename = "maxDuration")]
        pub max_duration: Int,
        #[serde(rename = "minRating")]
        pub min_rating: Float,
        #[serde(rename = "maxRating")]
        pub max_rating: Float,
        pub artists: Vec<Option<String>>,
        #[serde(rename = "noArtists")]
        pub no_artists: Vec<Option<String>>,
        pub albums: Vec<Option<String>>,
        #[serde(rename = "noAlbums")]
        pub no_albums: Vec<Option<String>>,
        pub titles: Vec<Option<String>>,
        #[serde(rename = "noTitles")]
        pub no_titles: Vec<Option<String>>,
        pub genres: Vec<Option<String>>,
        #[serde(rename = "noGenres")]
        pub no_genres: Vec<Option<String>>,
        pub keywords: Vec<Option<String>>,
        #[serde(rename = "noKeywords")]
        pub no_keywords: Vec<Option<String>>,
        pub shuffle: Boolean,
        pub limit: Int,
    }
}
impl graphql_client::GraphQLQuery for ListFilters {
    type Variables = list_filters::Variables;
    type ResponseData = list_filters::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: list_filters::QUERY,
            operation_name: list_filters::OPERATION_NAME,
        }
    }
}

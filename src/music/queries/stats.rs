pub struct Stats;
pub mod stats {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Stats";
    pub const QUERY : & str = "query Stats (\n    $min_duration: Int!,\n    $max_duration: Int!,\n    $min_rating: Float!,\n    $max_rating: Float!,\n    $artists: [String!]!,\n    $no_artists: [String!]!,\n    $albums: [String!]!,\n    $no_albums: [String!]!,\n    $titles: [String!]!,\n    $no_titles: [String!]!,\n    $genres: [String!]!,\n    $no_genres: [String!]!,\n    $keywords: [String!]!,\n    $no_keywords: [String!]!,\n    $shuffle: Boolean!,\n    $limit: Int!,\n) {\n    doStat (\n        minDuration: $min_duration\n        maxDuration: $max_duration\n        minRating: $min_rating\n        maxRating: $max_rating\n        artists: $artists\n        noArtists: $no_artists\n        albums: $albums\n        noAlbums: $no_albums\n        titles: $titles\n        noTitles: $no_titles\n        genres: $genres\n        noGenres: $no_genres\n        keywords: $keywords\n        noKeywords: $no_keywords\n        shuffle: $shuffle\n        limit: $limit\n    )\n    {\n        musics\n        artists\n        albums\n        links\n        genres\n        keywords\n        duration\n    }\n}\n" ;
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
    type BigInt = super::BigInt;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub min_duration: Int,
        pub max_duration: Int,
        pub min_rating: Float,
        pub max_rating: Float,
        pub artists: Vec<String>,
        pub no_artists: Vec<String>,
        pub albums: Vec<String>,
        pub no_albums: Vec<String>,
        pub titles: Vec<String>,
        pub no_titles: Vec<String>,
        pub genres: Vec<String>,
        pub no_genres: Vec<String>,
        pub keywords: Vec<String>,
        pub no_keywords: Vec<String>,
        pub shuffle: Boolean,
        pub limit: Int,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "doStat")]
        pub do_stat: Option<StatsDoStat>,
    }
    #[derive(Deserialize, Debug)]
    pub struct StatsDoStat {
        pub musics: BigInt,
        pub artists: BigInt,
        pub albums: BigInt,
        pub links: BigInt,
        pub genres: BigInt,
        pub keywords: BigInt,
        pub duration: BigInt,
    }
}
impl graphql_client::GraphQLQuery for Stats {
    type Variables = stats::Variables;
    type ResponseData = stats::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: stats::QUERY,
            operation_name: stats::OPERATION_NAME,
        }
    }
}

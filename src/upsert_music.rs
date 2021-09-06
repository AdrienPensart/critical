pub struct UpsertMusic;
pub mod upsert_music {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UpsertMusic";
    pub const QUERY : & str = "mutation UpsertMusic (\n    $title: String!,\n    $album: String!,\n    $artist: String!,\n    $genre: String!,\n    $user_id: Int!,\n    $rating: Float!,\n    $keywords: [String!]!,\n    $links: [String!]!,\n    $track: Int!,\n    $duration: Int!\n) {\n    upsertMusic(\n        where: {\n            title: $title\n            album: $album\n            artist: $artist\n            userId: $user_id\n        }\n        input: {\n            music: {\n                title: $title\n                album: $album\n                artist: $artist\n                duration: $duration\n                genre: $genre\n                keywords: $keywords\n                number: $track\n                rating: $rating\n                links: $links\n            }\n        }\n    )\n    {\n        clientMutationId\n    }\n}\n" ;
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
    pub struct Variables {
        pub title: String,
        pub album: String,
        pub artist: String,
        pub genre: String,
        pub user_id: Int,
        pub rating: Float,
        pub keywords: Vec<String>,
        pub links: Vec<String>,
        pub track: Int,
        pub duration: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "upsertMusic")]
        pub upsert_music: Option<UpsertMusicUpsertMusic>,
    }
    #[derive(Deserialize)]
    pub struct UpsertMusicUpsertMusic {
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for UpsertMusic {
    type Variables = upsert_music::Variables;
    type ResponseData = upsert_music::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: upsert_music::QUERY,
            operation_name: upsert_music::OPERATION_NAME,
        }
    }
}

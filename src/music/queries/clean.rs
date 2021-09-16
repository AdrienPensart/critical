pub struct CleanMusics;
pub mod clean_musics {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CleanMusics";
    pub const QUERY: &str =
        "mutation CleanMusics {\n    deleteAllMusic(input: {})\n    {\n        bigInt\n    }\n}\n";
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "deleteAllMusic")]
        pub delete_all_music: Option<CleanMusicsDeleteAllMusic>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CleanMusicsDeleteAllMusic {
        #[serde(rename = "bigInt")]
        pub big_int: Option<BigInt>,
    }
}
impl graphql_client::GraphQLQuery for CleanMusics {
    type Variables = clean_musics::Variables;
    type ResponseData = clean_musics::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: clean_musics::QUERY,
            operation_name: clean_musics::OPERATION_NAME,
        }
    }
}

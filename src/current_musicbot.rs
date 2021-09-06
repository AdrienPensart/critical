pub struct CurrentUserId;
pub mod current_user_id {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CurrentUserId";
    pub const QUERY: &str = "query CurrentUserId {\n    currentMusicbot\n}\n";
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
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "currentMusicbot")]
        pub current_musicbot: Option<Int>,
    }
}
impl graphql_client::GraphQLQuery for CurrentUserId {
    type Variables = current_user_id::Variables;
    type ResponseData = current_user_id::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: current_user_id::QUERY,
            operation_name: current_user_id::OPERATION_NAME,
        }
    }
}

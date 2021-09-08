pub struct Whoami;
pub mod whoami {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Whoami";
    pub const QUERY : & str = "query Whoami ($user_id: Int!) {\n  user(id: $user_id) {\n    firstName\n    id\n    lastName\n    createdAt\n    updatedAt\n  }\n}\n" ;
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
    type Datetime = super::Datetime;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub user_id: Int,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub user: Option<WhoamiUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct WhoamiUser {
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        pub id: Int,
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<Datetime>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<Datetime>,
    }
}
impl graphql_client::GraphQLQuery for Whoami {
    type Variables = whoami::Variables;
    type ResponseData = whoami::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: whoami::QUERY,
            operation_name: whoami::OPERATION_NAME,
        }
    }
}

pub struct Unregister;
pub mod unregister {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Unregister";
    pub const QUERY : & str = "mutation Unregister\n{\n    unregisterUser(input: {})\n    {\n        clientMutationId\n    }\n}\n" ;
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
        #[serde(rename = "unregisterUser")]
        pub unregister_user: Option<UnregisterUnregisterUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UnregisterUnregisterUser {
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for Unregister {
    type Variables = unregister::Variables;
    type ResponseData = unregister::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: unregister::QUERY,
            operation_name: unregister::OPERATION_NAME,
        }
    }
}

pub struct DeleteFilter;
pub mod delete_filter {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "DeleteFilter";
    pub const QUERY : & str = "mutation DeleteFilter($name: String!)\n{\n    deleteFilter(input: {name: $name})\n    {\n        clientMutationId\n    }\n}\n" ;
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
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "deleteFilter")]
        pub delete_filter: Option<DeleteFilterDeleteFilter>,
    }
    #[derive(Deserialize, Debug)]
    pub struct DeleteFilterDeleteFilter {
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for DeleteFilter {
    type Variables = delete_filter::Variables;
    type ResponseData = delete_filter::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: delete_filter::QUERY,
            operation_name: delete_filter::OPERATION_NAME,
        }
    }
}

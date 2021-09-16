pub struct CountFilters;
pub mod count_filters {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CountFilters";
    pub const QUERY: &str = "query CountFilters {\n    filters {\n        totalCount\n    }\n}\n";
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
        pub filters: Option<CountFiltersFilters>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CountFiltersFilters {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
}
impl graphql_client::GraphQLQuery for CountFilters {
    type Variables = count_filters::Variables;
    type ResponseData = count_filters::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: count_filters::QUERY,
            operation_name: count_filters::OPERATION_NAME,
        }
    }
}

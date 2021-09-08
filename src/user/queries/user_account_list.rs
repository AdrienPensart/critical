pub struct UserAccountList;
pub mod user_account_list {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserAccountList";
    pub const QUERY : & str = "query UserAccountList {\n    userAccountsList {\n        id\n        updatedAt\n        email\n        firstName\n        lastName\n        createdAt\n    }\n}\n" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "userAccountsList")]
        pub user_accounts_list: Option<Vec<UserAccountListUserAccountsList>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserAccountListUserAccountsList {
        pub id: Option<Int>,
        #[serde(rename = "updatedAt")]
        pub updated_at: Option<Datetime>,
        pub email: Option<String>,
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(rename = "createdAt")]
        pub created_at: Option<Datetime>,
    }
}
impl graphql_client::GraphQLQuery for UserAccountList {
    type Variables = user_account_list::Variables;
    type ResponseData = user_account_list::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_account_list::QUERY,
            operation_name: user_account_list::OPERATION_NAME,
        }
    }
}

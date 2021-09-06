pub struct Auth;
pub mod auth {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Auth";
    pub const QUERY : & str = "mutation Auth($email: String!, $password: String!) {\n    authenticate (input: {email: $email, password: $password}) {\n        jwtToken\n    }\n}\n" ;
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
    type JwtToken = super::JwtToken;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub email: String,
        pub password: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authenticate: Option<AuthAuthenticate>,
    }
    #[derive(Deserialize)]
    pub struct AuthAuthenticate {
        #[serde(rename = "jwtToken")]
        pub jwt_token: Option<JwtToken>,
    }
}
impl graphql_client::GraphQLQuery for Auth {
    type Variables = auth::Variables;
    type ResponseData = auth::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: auth::QUERY,
            operation_name: auth::OPERATION_NAME,
        }
    }
}

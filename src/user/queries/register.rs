pub struct Register;
pub mod register {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Register";
    pub const QUERY : & str = "mutation Register($first_name: String, $last_name: String, $email: String!, $password: String!)\n{\n    registerUser(input: {\n        firstName: $first_name,\n        lastName: $last_name,\n        email: $email,\n        password: $password\n    })\n    {\n       jwtToken\n    }\n}\n" ;
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
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub email: String,
        pub password: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "registerUser")]
        pub register_user: Option<RegisterRegisterUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct RegisterRegisterUser {
        #[serde(rename = "jwtToken")]
        pub jwt_token: Option<JwtToken>,
    }
}
impl graphql_client::GraphQLQuery for Register {
    type Variables = register::Variables;
    type ResponseData = register::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: register::QUERY,
            operation_name: register::OPERATION_NAME,
        }
    }
}

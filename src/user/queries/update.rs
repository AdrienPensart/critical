pub struct Update;
pub mod update {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Update";
    pub const QUERY : & str = "mutation Update($first_name: String, $last_name: String, $user_id: Int!)\n{\n    updateUser(input: {\n        patch: {\n            firstName: $first_name,\n            lastName: $last_name,\n        },\n        id: $user_id\n    })\n    {\n       clientMutationId\n    }\n}\n" ;
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
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub user_id: Int,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "updateUser")]
        pub update_user: Option<UpdateUpdateUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UpdateUpdateUser {
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for Update {
    type Variables = update::Variables;
    type ResponseData = update::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: update::QUERY,
            operation_name: update::OPERATION_NAME,
        }
    }
}

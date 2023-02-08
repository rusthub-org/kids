use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub phone_number: String,
    pub phone_public: bool,
    pub im_account: String,
    pub im_public: bool,
    pub website: String,
    pub topic_names: String,
    pub introduction: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignInInfo {
    pub signature: String,
    pub password: String,
}

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/home.graphql"
)]
pub struct HomeData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/home.graphql"
)]
pub struct RegisterData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/home.graphql"
)]
pub struct SignInData;

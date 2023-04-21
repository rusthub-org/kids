use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignStatus {
    pub sign_in: bool,
    pub username: String,
    pub token: String,
}

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct UserByIdData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct UserByUsernameData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct UserByUsernameDetailData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct UsersData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct UserUpdateOneFieldByIdData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/users.graphql"
)]
pub struct WishRandomData;

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topics.graphql"
)]
pub struct TopicsNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topics.graphql"
)]
pub struct TopicBySlugData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topics.graphql"
)]
pub struct TopicUserNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topics.graphql"
)]
pub struct TopicProjectNewData;

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/categories.graphql"
)]
pub struct CategoriesData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/categories.graphql"
)]
pub struct CategoryBySlugData;

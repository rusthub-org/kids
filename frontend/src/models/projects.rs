use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub user_id: String,
    pub category_id: String,
    pub subject: String,
    pub topic_names: String,
    pub investment: u64,
    pub currency_type: String,
    pub negotiated: bool,
    pub duration: u32,
    pub workday: bool,
    pub content: String,
    pub examples: String,
    pub files: String,
    pub worker_type: String,
    pub worker_info: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    pub language: String,
}

// -------------------------------
// GraphQLQuery for graphql_client
// -------------------------------

use graphql_client::GraphQLQuery;

type ObjectId = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByUserData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByCategoryData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByTopicData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByInvestmentData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByWorkerTypeData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectsByExternalData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectUpdateOneFieldByIdData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectRandomData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct FileNewData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../assets/graphql/schema.graphql",
    query_path = "../assets/graphql/projects.graphql"
)]
pub struct ProjectFileNewData;

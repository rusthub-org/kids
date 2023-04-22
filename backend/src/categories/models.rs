use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::{constant::GqlResult, pagination::ProjectsResult};

use crate::projects::services::projects_by_category_id;

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Category {
    pub _id: ObjectId,
    pub name_zh: String,
    pub name_en: String,
    pub slug: String,
}

#[async_graphql::ComplexObject]
impl Category {
    pub async fn projects(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects_by_category_id(
            db,
            self._id,
            1,
            String::from("-"),
            String::from("-"),
            1,
        )
        .await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct CategoryNew {
    pub name_zh: String,
    pub name_en: String,
    #[graphql(skip)]
    pub slug: String,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct CategoryUser {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct CategoryUserNew {
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}

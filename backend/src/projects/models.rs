use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::util::constant::{GqlResult, DTF_YMDHMSZ};
use crate::dbs::mongo::DataSource;

use crate::{
    categories::{self, models::Category},
    topics::{self, models::Topic},
    users::{self, models::User},
};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Project {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub category_id: ObjectId,
    pub subject: String,
    pub content: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub hits: u64,
    pub insides: u64,
    pub stars: u64,
    pub language: String,
    pub status: i8,
}

#[async_graphql::ComplexObject]
impl Project {
    pub async fn content_html(&self) -> String {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        // options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        // options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(&self.content, options);

        let mut content_html = String::new();
        html::push_html(&mut content_html, parser);

        content_html
    }

    pub async fn created_at_nyrsq(&self) -> String {
        self.created_at
            .to_chrono()
            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
            .format(DTF_YMDHMSZ)
            .to_string()
    }

    pub async fn updated_at_nyrsq(&self) -> String {
        self.updated_at
            .to_chrono()
            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
            .format(DTF_YMDHMSZ)
            .to_string()
    }

    pub async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_id(db, self.user_id).await
    }

    pub async fn category(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Category> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::category_by_id(db, self.category_id).await
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_project_id(db, self._id).await
    }

    pub async fn files(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<File>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        super::services::files_by_project_id(db, self._id).await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct ProjectNew {
    pub user_id: ObjectId,
    pub category_id: ObjectId,
    pub subject: String,
    pub content: String,
    pub contact_user: String,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_im: String,
    #[graphql(skip)]
    pub hits: u64,
    #[graphql(skip)]
    pub insides: u64,
    #[graphql(skip)]
    pub stars: u64,
    pub language: String,
    #[graphql(skip)]
    pub status: i8,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub _id: ObjectId,
    pub name: String,
    pub kind: i8,
    pub location: String,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct FileNew {
    pub name: String,
    pub kind: i8,
    pub location: String,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectFile {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub project_id: ObjectId,
    pub file_id: ObjectId,
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct ProjectFileNew {
    pub user_id: ObjectId,
    pub project_id: ObjectId,
    pub file_id: ObjectId,
}

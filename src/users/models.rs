use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::dbs::mongo::DataSource;
use crate::util::{
    constant::{GqlResult, DTF_YMDHMSZ},
    pagination::ProjectsResult,
};

use crate::{
    topics::{self, models::Topic},
    projects::services::projects_by_user_id,
};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub cred: String,
    pub nickname: String,
    pub phone_number: String,
    pub phone_public: bool,
    pub im_account: String,
    pub im_public: bool,
    pub website: String,
    pub introduction: String,
    pub worker_quality: i8,
    pub boss_quality: i8,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub status: i8,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn introduction_html(&self) -> String {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(&self.introduction, options);

        let mut introduction_html = String::new();
        html::push_html(&mut introduction_html, parser);

        introduction_html
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

    pub async fn keywords(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::keywords_by_user_id(db, self._id).await
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_user_id(db, self._id).await
    }

    pub async fn projects(
        &self,
        ctx: &async_graphql::Context<'_>,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects_by_user_id(
            db,
            self._id,
            1,
            String::from("-"),
            String::from("-"),
            status,
        )
        .await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct UserNew {
    pub username: String,
    pub email: String,
    pub cred: String,
    pub nickname: String,
    pub phone_number: String,
    pub phone_public: bool,
    pub im_account: String,
    pub im_public: bool,
    pub website: String,
    pub introduction: String,
    #[graphql(skip)]
    pub worker_quality: i8,
    #[graphql(skip)]
    pub boss_quality: i8,
    #[graphql(skip)]
    pub status: i8,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct SignInfo {
    pub username: String,
    pub token: String,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Wish {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub published: bool,
}

#[async_graphql::ComplexObject]
impl Wish {
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
        super::services::user_by_id(db, self.user_id).await
    }
}

#[derive(async_graphql::InputObject, Serialize, Deserialize)]
pub struct WishNew {
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    #[graphql(skip)]
    pub published: bool,
}

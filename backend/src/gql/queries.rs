use async_graphql::Context;
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::{
    constant::GqlResult,
    pagination::{UsersResult, ProjectsResult},
};

use crate::users::{
    self,
    models::{User, SignInfo, Wish},
};
use crate::projects::{
    self,
    models::{Project, File},
};
use crate::categories::{self, models::Category};
use crate::topics::{self, models::Topic};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // user sign in
    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        signature: String,
        password: String,
    ) -> GqlResult<SignInfo> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_sign_in(db, signature, password).await
    }

    // get user info by id
    async fn user_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_id(db, id).await
    }

    // get user info by email
    async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_email(db, email).await
    }

    // get user info by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_by_username(db, username).await
    }

    // Get all Users
    async fn users(
        &self,
        ctx: &Context<'_>,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<UsersResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::users(db, from_page, first_oid, last_oid, status).await
    }

    // Get all Users by worker_quality or boss_quality
    async fn users_by_quality(
        &self,
        ctx: &Context<'_>,
        quality_field: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<UsersResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::users_by_quality(
            db,
            quality_field,
            from_page,
            first_oid,
            last_oid,
            status,
        )
        .await
    }

    // Get project by its id
    async fn project_by_id(
        &self,
        ctx: &Context<'_>,
        project_id: ObjectId,
    ) -> GqlResult<Project> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::project_by_id(db, project_id).await
    }

    // get random project
    async fn project_random_id(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<ObjectId> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::project_random_id(db).await
    }

    // Get all projects
    async fn projects(
        &self,
        ctx: &Context<'_>,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects(db, from_page, first_oid, last_oid, status)
            .await
    }

    async fn projects_in_position(
        &self,
        ctx: &Context<'_>,
        username: String,
        position: String,
        limit: i64,
    ) -> GqlResult<Vec<Project>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_in_position(db, username, position, limit)
            .await
    }

    // Get all projects of one user by user_id
    async fn projects_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_user_id(
            db, user_id, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all projects of one user by username
    async fn projects_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_username(
            db, username, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all projects by category_id
    async fn projects_by_category_id(
        &self,
        ctx: &Context<'_>,
        category_id: ObjectId,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_category_id(
            db,
            category_id,
            from_page,
            first_oid,
            last_oid,
            status,
        )
        .await
    }

    // Get all projects by category_slug
    async fn projects_by_category_slug(
        &self,
        ctx: &Context<'_>,
        category_slug: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_category_slug(
            db,
            category_slug,
            from_page,
            first_oid,
            last_oid,
            status,
        )
        .await
    }

    // Get all projects by topic_id
    async fn projects_by_topic_id(
        &self,
        ctx: &Context<'_>,
        topic_id: ObjectId,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_topic_id(
            db, topic_id, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all projects by topic_slug
    async fn projects_by_topic_slug(
        &self,
        ctx: &Context<'_>,
        topic_slug: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_topic_slug(
            db, topic_slug, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // Get all projects by investment
    async fn projects_by_investment(
        &self,
        ctx: &Context<'_>,
        investment_min: i64,
        investment_max: i64,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_investment(
            db,
            investment_min,
            investment_max,
            from_page,
            first_oid,
            last_oid,
            status,
        )
        .await
    }

    // Get all projects by worker_type
    async fn projects_by_worker_type(
        &self,
        ctx: &Context<'_>,
        worker_type: String,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_worker_type(
            db,
            worker_type,
            from_page,
            first_oid,
            last_oid,
            status,
        )
        .await
    }

    // Get all projects by external
    async fn projects_by_external(
        &self,
        ctx: &Context<'_>,
        external: bool,
        from_page: u32,
        first_oid: String,
        last_oid: String,
        status: i8,
    ) -> GqlResult<ProjectsResult> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::projects_by_external(
            db, external, from_page, first_oid, last_oid, status,
        )
        .await
    }

    // get file by id
    async fn file_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::file_by_id(db, id).await
    }

    // get all files of one project by project_id
    async fn files_by_project_id(
        &self,
        ctx: &Context<'_>,
        project_id: ObjectId,
    ) -> GqlResult<Vec<File>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::files_by_project_id(db, project_id).await
    }

    // Get all categories
    async fn categories(&self, ctx: &Context<'_>) -> GqlResult<Vec<Category>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::categories(db).await
    }

    // Get all categories by user_id
    async fn categories_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Category>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::categories_by_user_id(db, user_id).await
    }

    // Get all categories by username
    async fn categories_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Category>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::categories_by_username(db, username).await
    }

    // Get category by its id
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<Category> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::category_by_id(db, id).await
    }

    // Get category by its slug
    async fn category_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> GqlResult<Category> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::category_by_slug(db, slug).await
    }

    // get topic info by id
    async fn topic_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_by_id(db, id).await
    }

    // get topic info by slug
    async fn topic_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_by_slug(db, slug).await
    }

    // get all topics
    async fn topics(&self, ctx: &Context<'_>) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics(db).await
    }

    // get topics by project_id
    async fn topics_by_project_id(
        &self,
        ctx: &Context<'_>,
        project_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_project_id(db, project_id).await
    }

    // get users' keywords by user_id
    async fn keywords_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::keywords_by_user_id(db, user_id).await
    }

    // get users' keywords by username
    async fn keywords_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::keywords_by_username(db, username).await
    }

    // get topics by user_id
    async fn topics_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_user_id(db, user_id).await
    }

    // get topics by username
    async fn topics_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_by_username(db, username).await
    }

    // get all wishes
    async fn wishes(
        &self,
        ctx: &Context<'_>,
        published: i8,
    ) -> GqlResult<Vec<Wish>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wishes(db, published).await
    }

    // get random wish
    async fn wish_random(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Wish> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wish_random(db, username).await
    }
}

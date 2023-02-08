use async_graphql::Context;
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;

use crate::users::{
    self,
    models::{User, UserNew, Wish, WishNew},
};
use crate::projects::{
    self,
    models::{Project, ProjectNew, File, FileNew, ProjectFile, ProjectFileNew},
};
use crate::categories::{
    self,
    models::{Category, CategoryNew, CategoryUser, CategoryUserNew},
};
use crate::topics::{
    self,
    models::{
        Topic, TopicNew, TopicUser, TopicUserNew, TopicProject, TopicProjectNew,
    },
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_register(db, user_new).await
    }

    // Change user password
    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        pwd_cur: String,
        pwd_new: String,
        token: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_change_password(db, pwd_cur, pwd_new, token).await
    }

    // update user profile
    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
        token: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_update_profile(db, user_new, token).await
    }

    // modify user's one field by its id
    async fn user_update_one_field_by_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
        field_name: String,
        field_val: String,
    ) -> GqlResult<User> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::user_update_one_field_by_id(
            db, user_id, field_name, field_val,
        )
        .await
    }

    // Add new project
    async fn project_new(
        &self,
        ctx: &Context<'_>,
        project_new: ProjectNew,
    ) -> GqlResult<Project> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::project_new(db, project_new).await
    }

    // modify project's one field by its id
    async fn project_update_one_field_by_id(
        &self,
        ctx: &Context<'_>,
        project_id: ObjectId,
        field_name: String,
        field_val: String,
    ) -> GqlResult<Project> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::project_update_one_field_by_id(
            db, project_id, field_name, field_val,
        )
        .await
    }

    // Add new file
    async fn file_new(
        &self,
        ctx: &Context<'_>,
        file_new: FileNew,
    ) -> GqlResult<File> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::file_new(db, file_new).await
    }

    // Add new project_file
    async fn project_file_new(
        &self,
        ctx: &Context<'_>,
        project_file_new: ProjectFileNew,
    ) -> GqlResult<ProjectFile> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        projects::services::project_file_new(db, project_file_new).await
    }

    // Add new category
    async fn category_new(
        &self,
        ctx: &Context<'_>,
        category_new: CategoryNew,
    ) -> GqlResult<Category> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::category_new(db, category_new).await
    }

    // Add new category
    async fn category_user_new(
        &self,
        ctx: &Context<'_>,
        category_user_new: CategoryUserNew,
    ) -> GqlResult<CategoryUser> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        categories::services::category_user_new(db, category_user_new).await
    }

    // Add new topic
    async fn topic_new(
        &self,
        ctx: &Context<'_>,
        topic_new: TopicNew,
    ) -> GqlResult<Topic> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_new(db, topic_new).await
    }

    // Add new topics
    async fn topics_new(
        &self,
        ctx: &Context<'_>,
        topic_names: String,
    ) -> GqlResult<Vec<Topic>> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topics_new(db, topic_names).await
    }

    // Add new topic_user
    async fn topic_user_new(
        &self,
        ctx: &Context<'_>,
        topic_user_new: TopicUserNew,
    ) -> GqlResult<TopicUser> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_user_new(db, topic_user_new).await
    }

    // Add new topic_project
    async fn topic_project_new(
        &self,
        ctx: &Context<'_>,
        topic_project_new: TopicProjectNew,
    ) -> GqlResult<TopicProject> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        topics::services::topic_project_new(db, topic_project_new).await
    }

    // Add new wish
    async fn wish_new(
        &self,
        ctx: &Context<'_>,
        wish_new: WishNew,
    ) -> GqlResult<Wish> {
        let db = &ctx.data_unchecked::<DataSource>().db;
        users::services::wish_new(db, wish_new).await
    }
}

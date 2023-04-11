use tide::{self, Server};

pub mod home;
pub mod users;
pub mod projects;
pub mod categories;
pub mod topics;
pub mod admin;

use crate::State;
use crate::util::common::tpls_dir;

pub async fn push_res(app: &mut Server<State>) {
    app.at("/").get(super::routes::home::init);

    app.at("/static/*").serve_dir("../assets/static/").unwrap();

    app.at("/ads.txt")
        .serve_file(format!("{}{}", tpls_dir().await, "ads.txt"))
        .unwrap_or_default();
    app.at("/sitemap.txt")
        .serve_file(format!("{}{}", tpls_dir().await, "sitemap.txt"))
        .unwrap_or_default();

    let mut admin = app.at("/admin");
    admin.at("/").get(super::routes::admin::admin_index);
    admin.at("/projects/:division").get(super::routes::admin::projects_admin);
    admin.at("/project/:project_id").get(super::routes::admin::project_admin);
    admin
        .at("/project/:project_id/:field_name/:field_val")
        .get(super::routes::admin::project_update_one_field);

    let mut home = app.at("/:language");
    home.at("/").get(super::routes::home::index);
    home.at("/register")
        .get(super::routes::home::register)
        .post(super::routes::home::register);
    home.at("/sign-in")
        .get(super::routes::home::sign_in)
        .post(super::routes::home::sign_in);
    home.at("/sign-out").get(super::routes::home::sign_out);

    let mut users = home.at("/users");
    users.at("/").get(super::routes::users::users_index);
    users.at("/:filter_str").get(super::routes::users::users_filter);

    let mut user = home.at("/user");
    user.at("/:user_id/activate")
        .get(super::routes::users::user_activate)
        .post(super::routes::users::user_activate);
    user.at("/:author_username").get(super::routes::users::user_index);
    user.at("/:author_username/projects")
        .get(super::routes::projects::projects_by_user);

    let mut projects = home.at("/projects");
    projects.at("/").get(super::routes::projects::projects_index);
    // projects
    //     .at("/:filter_str")
    //     .get(super::routes::projects::projects_filter)
    //     .post(super::routes::projects::projects_filter);

    let mut project = home.at("/project");
    project.at("/").get(super::routes::projects::project_random);
    project
        .at("/new")
        .get(super::routes::projects::project_new)
        .post(super::routes::projects::project_new);
    project.at("/:project_id").get(super::routes::projects::project_index);
    project
        .at("/file/new/:file_name/:file_kind")
        .put(super::routes::projects::file_new);

    // let mut categories = app.at("/categories");
    let mut category = home.at("/category");
    category
        .at("/:category_slug/projects")
        .get(super::routes::projects::projects_by_category);

    // let mut topics = app.at("/topics");
    let mut topic = home.at("/topic");
    topic
        .at("/:topic_slug/projects")
        .get(super::routes::projects::projects_by_topic);
}

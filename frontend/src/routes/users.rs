use std::collections::BTreeMap;
use tide::{Request, http::Method};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;

use crate::State;
use crate::util::{
    common::{gql_uri, sign_status},
    email::send_email,
    tpl::{Hbs, insert_user_by_username, insert_wish_random},
};

use crate::models::{
    Page,
    users::{
        UsersData, users_data, UsersByQualityData, users_by_quality_data,
        UserByIdData, user_by_id_data, UserByUsernameDetailData,
        user_by_username_detail_data, UserUpdateOneFieldByIdData,
        user_update_one_field_by_id_data,
    },
};

pub async fn users_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut users_index_tpl: Hbs = Hbs::new("users/users-index").await;
    users_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    users_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    data.insert("users-all-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let page: Page = req.query()?;
    let users_build_query = UsersData::build_query(users_data::Variables {
        from_page: page.from,
        first_oid: page.first,
        last_oid: page.last,
        status: 1,
    });
    let users_query = json!(users_build_query);

    let users_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(users_query).recv_json().await?;
    let users_resp_data = users_resp_body.data.expect("无响应数据");

    let users = users_resp_data["users"].clone();
    data.insert("pagination", users);

    users_index_tpl.render(&data).await
}

pub async fn users_filter(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut users_filter_tpl: Hbs = Hbs::new("users/users-index").await;
    users_filter_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_pagination()
        .await
        .reg_footer()
        .await;
    users_filter_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let filter_str = req.param("filter_str")?;
    let page: Page = req.query()?;

    let filter_desc;
    let quality_field = match filter_str {
        "boss" => {
            data.insert("users-boss-selected", json!("is-selected"));
            filter_desc = json!({
                "condition": "user-filter-boss",
                "content": ""
            });

            "boss_quality"
        }
        _ => {
            data.insert("users-worker-selected", json!("is-selected"));
            filter_desc = json!({
                "condition": "user-filter-worker",
                "content": ""
            });

            "worker_quality"
        }
    };
    data.insert("filter_desc", filter_desc);

    let users_by_quality_build_query =
        UsersByQualityData::build_query(users_by_quality_data::Variables {
            quality_field: String::from(quality_field),
            from_page: page.from,
            first_oid: page.first,
            last_oid: page.last,
            status: 1,
        });
    let users_by_quality_query = json!(users_by_quality_build_query);

    let users_by_quality_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(users_by_quality_query)
            .recv_json()
            .await
            .unwrap();
    let users_by_quality_resp_data =
        users_by_quality_resp_body.data.expect("无响应数据");

    let users_by_quality = users_by_quality_resp_data["usersByQuality"].clone();
    data.insert("pagination", users_by_quality);

    users_filter_tpl.render(&data).await
}

pub async fn user_index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut user_index_tpl: Hbs = Hbs::new("users/users-user-detail").await;
    user_index_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_index_tpl
        .reg_script_values()
        .await
        .reg_script_ops()
        .await
        .reg_script_lang()
        .await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        data.insert("sign-in", json!(sign_status.sign_in));
        insert_user_by_username(sign_status.username, &mut data).await;
    }

    let author_username = req.param("author_username")?;
    let author_by_username_detail_build_query =
        UserByUsernameDetailData::build_query(
            user_by_username_detail_data::Variables {
                username: String::from(author_username),
            },
        );
    let author_by_username_detail_query =
        json!(author_by_username_detail_build_query);

    let author_by_username_detail_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(author_by_username_detail_query)
            .recv_json()
            .await
            .unwrap();
    let author_by_username_detail_resp_data =
        author_by_username_detail_resp_body.data.expect("无响应数据");

    let author_user_detail =
        author_by_username_detail_resp_data["userByUsername"].clone();
    data.insert("author_user", author_user_detail);

    user_index_tpl.render(&data).await
}

pub async fn user_activate(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut user_activate_tpl: Hbs =
        Hbs::new("users/users-user-activate").await;
    user_activate_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    user_activate_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("nav-users-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    let user_id = req.param("user_id")?;
    match req.method() {
        Method::Post => {
            let user_resend_build_query =
                UserByIdData::build_query(user_by_id_data::Variables {
                    id: user_id.to_string(),
                });
            let user_resend_query = json!(user_resend_build_query);

            let user_resend_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(user_resend_query)
                    .recv_json()
                    .await?;
            let user_resend_resp_data =
                user_resend_resp_body.data.expect("无响应数据");

            let user_resend = user_resend_resp_data["userById"].clone();

            send_email(
                language,
                user_id.to_string(),
                user_resend["username"].as_str().unwrap().to_string(),
                user_resend["nickname"].as_str().unwrap().to_string(),
                user_resend["email"].as_str().unwrap().to_string(),
            )
            .await;

            data.insert("user_resend", user_resend);
        }
        _ => {
            let user_worker_quality_build_query =
                UserUpdateOneFieldByIdData::build_query(
                    user_update_one_field_by_id_data::Variables {
                        user_id: user_id.to_string(),
                        field_name: String::from("worker_quality"),
                        field_val: String::from("10"),
                    },
                );
            let user_worker_quality_query =
                json!(user_worker_quality_build_query);

            let _user_worker_quality_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(user_worker_quality_query)
                    .recv_json()
                    .await?;

            let user_activate_build_query =
                UserUpdateOneFieldByIdData::build_query(
                    user_update_one_field_by_id_data::Variables {
                        user_id: user_id.to_string(),
                        field_name: String::from("status"),
                        field_val: String::from("1"),
                    },
                );
            let user_activate_query = json!(user_activate_build_query);

            let user_activate_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(user_activate_query)
                    .recv_json()
                    .await?;
            let user_activate_resp_data =
                user_activate_resp_body.data.expect("无响应数据");

            let user_activate =
                user_activate_resp_data["userUpdateOneFieldById"].clone();

            data.insert("user_activate", user_activate);
        }
    }

    user_activate_tpl.render(&data).await
}

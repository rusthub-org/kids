use std::collections::BTreeMap;
use tide::{
    Request, Response, Redirect,
    http::{Method, Cookie},
};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::{json, Value};

use crate::State;
use crate::util::{
    constant::CFG,
    common::gql_uri,
    email::send_email,
    tpl::{Hbs, insert_wish_random},
};

use crate::models::{
    home::{
        HomeData, home_data, RegisterInfo, SignInInfo, RegisterData,
        register_data, SignInData, sign_in_data,
    },
    topics::{
        TopicsNewData, topics_new_data, TopicUserNewData, topic_user_new_data,
    },
};

pub async fn init(req: Request<State>) -> tide::Result {
    let accept_language = req.header("accept-language");
    let language =
        String::from(if let Some(client_language) = accept_language {
            if client_language.as_str().starts_with("zh") {
                "zh-cn"
            } else {
                "en-us"
            }
        } else {
            "en-us"
        });

    let (init_tpl, data) = init_index(language).await;

    init_tpl.render(&data).await
}

pub async fn index(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let (index_tpl, data) = init_index(language).await;

    index_tpl.render(&data).await
}

async fn init_index<'ii>(
    language: String,
) -> (Hbs<'ii>, BTreeMap<&'ii str, Value>) {
    let mut tpl: Hbs = Hbs::new("index").await;
    tpl.reg_head().await.reg_container().await.reg_footer().await;
    tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    insert_wish_random(&mut data).await;

    // insert home data
    let home_build_query = HomeData::build_query(home_data::Variables {
        username: "-".to_string(),
    });
    let home_query = json!(home_build_query);

    let home_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(home_query)
            .recv_json()
            .await
            .unwrap();
    let home_resp_data = home_resp_body.data.expect("无响应数据");

    // let managed_projects = home_resp_data["managedProjects"].clone();
    // data.insert("managed_projects", managed_projects);

    let recommended_projects = home_resp_data["recommendedProjects"].clone();
    data.insert("recommended_projects", recommended_projects);

    let published_projects = home_resp_data["publishedProjects"].clone();
    data.insert("published_projects", published_projects);

    (tpl, data)
}

pub async fn register(mut req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut register_tpl: Hbs = Hbs::new("register").await;
    register_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    register_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("register-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    if req.method().eq(&Method::Post) {
        let register_info: RegisterInfo = req.body_form().await?;

        let build_query = RegisterData::build_query(register_data::Variables {
            username: register_info.username.clone(),
            email: register_info.email.clone(),
            cred: register_info.password,
            nickname: register_info.nickname.clone(),
            phone_number: register_info.phone_number,
            phone_public: register_info.phone_public,
            im_account: register_info.im_account,
            im_public: register_info.im_public,
            website: register_info.website,
            introduction: register_info.introduction,
        });
        let query = json!(build_query);

        let resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await).body(query).recv_json().await?;
        let resp_data = resp_body.data;

        if let Some(register_val) = resp_data {
            let register_result = register_val["userRegister"].clone();
            let user_id = register_result["id"].as_str().unwrap();

            // create topics
            let topics_build_query =
                TopicsNewData::build_query(topics_new_data::Variables {
                    topic_names: register_info.topic_names,
                });
            let topics_query = json!(topics_build_query);

            let topics_resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await)
                    .body(topics_query)
                    .recv_json()
                    .await?;
            let topics_resp_data = topics_resp_body.data;

            if let Some(topics_info) = topics_resp_data {
                let topic_ids = topics_info["topicsNew"].as_array().unwrap();
                for topic_id in topic_ids {
                    let topic_id = topic_id["id"].as_str().unwrap();
                    let topic_user_build_query = TopicUserNewData::build_query(
                        topic_user_new_data::Variables {
                            user_id: user_id.to_string(),
                            topic_id: topic_id.to_string(),
                        },
                    );
                    let topic_user_query = json!(topic_user_build_query);
                    let _topic_user_resp_body: GqlResponse<serde_json::Value> =
                        surf::post(&gql_uri().await)
                            .body(topic_user_query)
                            .recv_json()
                            .await?;
                }
            }

            send_email(
                language,
                user_id.to_string(),
                register_info.username,
                register_info.nickname,
                register_info.email,
            )
            .await;

            data.insert("register_result", register_result);
        } else {
            data.insert(
                "register_failed",
                json!(resp_body.errors.unwrap()[0].message),
            );
        }
    }

    register_tpl.render(&data).await
}

pub async fn sign_in(mut req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);

    let mut sign_in_tpl: Hbs = Hbs::new("sign-in").await;
    sign_in_tpl
        .reg_head()
        .await
        .reg_header()
        .await
        .reg_container()
        .await
        .reg_footer()
        .await;
    sign_in_tpl.reg_script_values().await.reg_script_lang().await;

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    data.insert("language", json!(language));
    data.insert("sign-in-nav-selected", json!("is-selected"));
    insert_wish_random(&mut data).await;

    match req.method() {
        Method::Post => {
            let sign_in_info: SignInInfo = req.body_form().await?;

            let build_query =
                SignInData::build_query(sign_in_data::Variables {
                    signature: sign_in_info.signature,
                    password: sign_in_info.password,
                });
            let query = json!(build_query);

            let resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await).body(query).recv_json().await?;
            let resp_data = resp_body.data;

            if let Some(sign_in_val) = resp_data {
                let sign_in_user = sign_in_val["userSignIn"].clone();

                let mut resp: Response =
                    Redirect::new(format!("/{}/projects", language)).into();

                let mut username_cookie = Cookie::new(
                    "username",
                    String::from(sign_in_user["username"].as_str().unwrap()),
                );
                set_cookie(&mut username_cookie).await;
                resp.insert_cookie(username_cookie);

                let mut token_cookie = Cookie::new(
                    "token",
                    String::from(sign_in_user["token"].as_str().unwrap()),
                );
                set_cookie(&mut token_cookie).await;
                resp.insert_cookie(token_cookie);

                Ok(resp.into())
            } else {
                let error = resp_body.errors.unwrap()[0].clone();
                data.insert("sign_in_failed", json!(error.message));

                if let Some(eev) = error.extensions {
                    let sign_in_failed_user_id =
                        eev.get("user_id").unwrap().as_str().unwrap();
                    data.insert(
                        "sign_in_failed_user_id",
                        json!(sign_in_failed_user_id),
                    );
                }

                sign_in_tpl.render(&data).await
            }
        }
        _ => sign_in_tpl.render(&data).await,
    }
}

pub async fn sign_out(req: Request<State>) -> tide::Result {
    let language = String::from(req.param("language")?);
    let mut resp: Response = Redirect::new(format!("/{}", language)).into();

    let username_cookie = req.cookie("username");
    if let Some(mut cookie) = username_cookie {
        set_cookie(&mut cookie).await;
        resp.remove_cookie(cookie);
    }

    let token_cookie = req.cookie("token");
    if let Some(mut cookie) = token_cookie {
        set_cookie(&mut cookie).await;
        resp.remove_cookie(cookie);
    }

    Ok(resp.into())
}

async fn set_cookie<'c>(cookie: &mut Cookie<'c>) {
    let domain = CFG.get("DOMAIN").unwrap();

    cookie.set_domain(domain);
    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_http_only(true);
}

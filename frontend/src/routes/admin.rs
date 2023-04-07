use std::collections::BTreeMap;
use tide::{Request, Response, Redirect};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde_json::json;

use crate::State;
use crate::util::{
    common::{gql_uri, sign_status},
    tpl::{Hbs, insert_user_by_username},
};

use crate::models::{
    Page,
    projects::{
        ProjectsByExternalData, projects_by_external_data, ProjectData,
        project_data, ProjectUpdateOneFieldByIdData,
        project_update_one_field_by_id_data,
    },
};

pub async fn admin_index(req: Request<State>) -> tide::Result {
    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut admin_index_tpl: Hbs = Hbs::new("admin/admin-index").await;
        admin_index_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        admin_index_tpl.reg_script_values().await.reg_script_lang().await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        admin_index_tpl.render(&data).await
    } else {
        let resp: Response = Redirect::new("/zh-cn/sign-in").into();

        Ok(resp.into())
    }
}

pub async fn projects_admin(req: Request<State>) -> tide::Result {
    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut admin_projects_tpl: Hbs =
            Hbs::new("admin/admin-projects").await;
        admin_projects_tpl
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
        admin_projects_tpl.reg_script_values().await.reg_script_lang().await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        let division = req.param("division")?.trim().eq("external");
        let page: Page = req.query()?;
        let projects_by_external_build_query =
            ProjectsByExternalData::build_query(
                projects_by_external_data::Variables {
                    external: division,
                    from_page: page.from,
                    first_oid: page.first,
                    last_oid: page.last,
                    status: 0,
                },
            );
        let projects_by_external_query =
            json!(projects_by_external_build_query);

        let projects_by_external_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(projects_by_external_query)
                .recv_json()
                .await
                .unwrap();
        let projects_by_external_resp_data =
            projects_by_external_resp_body.data.expect("无响应数据");

        let projects_by_external =
            projects_by_external_resp_data["projectsByExternal"].clone();
        data.insert("pagination", projects_by_external);

        admin_projects_tpl.render(&data).await
    } else {
        let resp: Response = Redirect::new("/zh-cn/sign-in").into();

        Ok(resp.into())
    }
}

pub async fn project_admin(req: Request<State>) -> tide::Result {
    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let mut project_index_tpl: Hbs =
            Hbs::new("admin/admin-project-detail").await;
        project_index_tpl
            .reg_head()
            .await
            .reg_header()
            .await
            .reg_container()
            .await
            .reg_footer()
            .await;
        project_index_tpl
            .reg_script_values()
            .await
            .reg_script_ops()
            .await
            .reg_script_lang()
            .await;

        let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
        data.insert("language", json!("zh-cn"));
        data.insert("nav-admin-selected", json!("is-selected"));
        insert_user_by_username(sign_status.username, &mut data).await;

        let project_id = req.param("project_id")?;

        let project_update_hits_build_query =
            ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: project_id.to_string(),
                    field_name: String::from("hits"),
                    field_val: String::from("3"),
                },
            );
        let project_update_hits_query = json!(project_update_hits_build_query);
        let _project_update_hits_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(project_update_hits_query)
                .recv_json()
                .await?;

        let project_build_query =
            ProjectData::build_query(project_data::Variables {
                project_id: project_id.to_string(),
            });
        let project_query = json!(project_build_query);

        let project_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(project_query)
                .recv_json()
                .await?;
        let project_resp_data = project_resp_body.data.expect("无响应数据");

        let project = project_resp_data["projectById"].clone();
        data.insert("project", project);

        project_index_tpl.render(&data).await
    } else {
        let resp: Response = Redirect::new("/zh-cn/sign-in").into();

        Ok(resp.into())
    }
}

pub async fn project_update_one_field(req: Request<State>) -> tide::Result {
    let sign_status = sign_status(&req).await;
    if sign_status.sign_in {
        let project_id = req.param("project_id")?;
        let field_name = req.param("field_name")?;
        let field_val = req.param("field_val")?;

        let project_update_hits_build_query =
            ProjectUpdateOneFieldByIdData::build_query(
                project_update_one_field_by_id_data::Variables {
                    project_id: String::from(project_id),
                    field_name: String::from(field_name),
                    field_val: String::from(field_val),
                },
            );
        let project_update_hits_query = json!(project_update_hits_build_query);
        let _project_update_hits_resp_body: GqlResponse<serde_json::Value> =
            surf::post(&gql_uri().await)
                .body(project_update_hits_query)
                .recv_json()
                .await?;

        let resp: Response =
            Redirect::new(format!("/admin/project/{}", project_id)).into();

        Ok(resp.into())
    } else {
        let resp: Response = Redirect::new("/zh-cn/sign-in").into();

        Ok(resp.into())
    }
}

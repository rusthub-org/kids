use std::collections::BTreeMap;
use tide::{Response, StatusCode, Body, http::mime::HTML};
use handlebars::{Handlebars, Context, Helper, Output, RenderContext, RenderError};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use serde::Serialize;
use serde_json::json;

use super::common::{gql_uri, scripts_dir, tpls_dir, get_lang_msg};

use crate::models::users::{
    UserByUsernameData, user_by_username_data, WishRandomData, wish_random_data,
};

pub struct Hbs<'hbs> {
    pub name: String,
    pub reg: Handlebars<'hbs>,
}

impl<'hbs> Hbs<'hbs> {
    pub async fn new(rel_path: &str) -> Hbs<'hbs> {
        let tpl_name = rel_path.replace("/", "_");
        let abs_path = format!("{}{}.html", &tpls_dir().await, &rel_path);

        // create the handlebars registry
        let mut hbs_reg = Handlebars::new();
        // register template from a file and assign a name to it
        hbs_reg.register_template_file(&tpl_name, &abs_path).unwrap();

        Hbs { name: tpl_name, reg: hbs_reg }
    }

    pub async fn render<T>(&self, data: &T) -> tide::Result
    where
        T: Serialize,
    {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_content_type(HTML);
        resp.set_body(Body::from_string(self.reg.render(&self.name, data)?));

        Ok(resp.into())
    }

    pub async fn reg_head(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "head",
                format!("{}{}", tpls_dir().await, "common/head.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_header(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "nav-global",
                format!("{}{}", tpls_dir().await, "common/nav-global.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign",
                format!("{}{}", tpls_dir().await, "common/sign.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign-popover",
                format!("{}{}", tpls_dir().await, "common/sign-popover.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "header",
                format!("{}{}", tpls_dir().await, "common/header.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_container(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "wish-random",
                format!("{}{}", tpls_dir().await, "common/wish-random.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_pagination(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "pagination",
                format!("{}{}", tpls_dir().await, "common/pagination.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_footer(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "footer",
                format!("{}{}", tpls_dir().await, "common/footer.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_values(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "helper-values",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "values/helper-values.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cmp",
                format!("{}{}", scripts_dir().await, "values/str-cmp.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cut",
                format!("{}{}", scripts_dir().await, "values/str-cut.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "value-check",
                format!("{}{}", scripts_dir().await, "values/value-check.rhai"),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_ops(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "add-op",
                format!("{}{}", scripts_dir().await, "ops/add-op.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "level-op",
                format!("{}{}", scripts_dir().await, "ops/level-op.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "sci-format",
                format!("{}{}", scripts_dir().await, "ops/sci-format.rhai"),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_lang(&mut self) -> &mut Hbs<'hbs> {
        self.reg.register_helper("lang", Box::new(lang_helper));

        self
    }
}

fn lang_helper(
    helper: &Helper,
    _hbs: &Handlebars,
    c: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let lang_id = if let Some(language_val) = c.data().get("language") {
        language_val.as_str().unwrap_or_default()
    } else {
        "en-us"
    };

    let root_tpl = rc.get_root_template_name().unwrap().as_str();

    let msg_id = if let Some(msg_json) = helper.param(0) {
        msg_json.value().as_str().unwrap_or_default()
    } else {
        "{{ lang }} must have at least one parameter"
    };
    let msg_args = if helper.params().len() > 1 {
        Some(helper.param(1).unwrap().value().as_object().unwrap())
    } else {
        None
    };

    let value = get_lang_msg(lang_id, root_tpl, msg_id, msg_args);
    out.write(&value)?;

    Ok(())
}

pub async fn insert_user_by_username(
    sign_username: String,
    data: &mut BTreeMap<&str, serde_json::Value>,
) {
    let user_by_username_build_query =
        UserByUsernameData::build_query(user_by_username_data::Variables {
            username: sign_username,
        });
    let user_by_username_query = json!(user_by_username_build_query);

    let user_by_username_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(user_by_username_query)
            .recv_json()
            .await
            .unwrap();
    let user_by_username_resp_data =
        user_by_username_resp_body.data.expect("无响应数据");

    let user = user_by_username_resp_data["userByUsername"].clone();
    data.insert("user", user);
}

pub async fn insert_wish_random(data: &mut BTreeMap<&str, serde_json::Value>) {
    let wish_random_build_query =
        WishRandomData::build_query(wish_random_data::Variables {
            username: "-".to_string(),
        });
    let wish_random_query = json!(wish_random_build_query);

    let wish_random_resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await)
            .body(wish_random_query)
            .recv_json()
            .await
            .unwrap();
    let wish_random_resp_data = wish_random_resp_body.data.expect("无响应数据");

    let wish = wish_random_resp_data["wishRandom"].clone();
    data.insert("wish", wish);
}

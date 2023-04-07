use std::{path::PathBuf, fs::read_to_string};
use tide::Request;
use fluent_bundle::{FluentBundle, FluentResource, FluentArgs, FluentValue};
use serde_json::{Map, Value};

use crate::State;
use crate::util::constant::CFG;

use crate::models::users::SignStatus;

pub async fn gql_uri() -> String {
    let gql_prot = CFG.get("GQL_PROT").unwrap();
    let gql_addr = CFG.get("GQL_ADDR").unwrap();
    let gql_port = CFG.get("GQL_PORT").unwrap();
    let gql_uri = CFG.get("GQL_URI").unwrap();
    let gql_path = CFG.get("GQL_VER").unwrap();

    format!("{}://{}:{}/{}/{}", gql_prot, gql_addr, gql_port, gql_uri, gql_path)
}

pub async fn scripts_dir() -> String {
    format!("../assets/{}/", "scripts")
}

pub async fn tpls_dir() -> String {
    format!("./{}/", "templates")
}

pub async fn sign_status(req: &Request<State>) -> SignStatus {
    let username = if let Some(cookie) = req.cookie("username") {
        String::from(cookie.value())
    } else {
        String::from("-")
    };

    let token = if let Some(cookie) = req.cookie("token") {
        String::from(cookie.value())
    } else {
        String::from("-")
    };

    let sign_in = if let "" | "-" = username.trim() { false } else { true };

    SignStatus { sign_in, username, token }
}

pub fn get_lang_msg(
    lang_id: &str,
    root_tpl: &str,
    msg_id: &str,
    msg_args: Option<&Map<String, Value>>,
) -> String {
    let mut bundle = FluentBundle::default();

    let lang_res = get_lang_res(root_tpl);
    for res_file in lang_res {
        let res_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../assets/locales")
            .join(lang_id)
            .join(res_file);
        let res_source = read_to_string(&res_path).expect(&format!(
            "Failed to read res file: {}.",
            res_path.to_str().unwrap()
        ));

        let resource = FluentResource::try_new(res_source.to_owned())
            .expect(&format!("{}, could not parse a LANG string.", res_source));
        bundle
            .add_resource(resource)
            .expect("Failed to add LANG resources to the bundle.");
    }

    let mut args = FluentArgs::new();
    if let Some(msg_map) = msg_args {
        for arg_key in msg_map.keys() {
            let arg_val =
                msg_map.get(arg_key).unwrap().as_str().unwrap_or_default();
            args.set(arg_key, FluentValue::from(arg_val));
        }
    }

    let msg = bundle
        .get_message(msg_id)
        .expect(format!("{} is not exists", msg_id).as_str());
    let pattern =
        msg.value().expect(format!("{} must have a value", msg_id).as_str());

    let mut errors = vec![];
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);

    value.to_string()
}

fn get_lang_res(root_tpl: &str) -> Vec<&str> {
    match root_tpl {
        "index" | "register" | "sign-in" | "admin_admin-index" => {
            vec!["common.lang", "home.lang"]
        }
        "projects_projects-index"
        | "projects_projects-project-detail"
        | "projects_projects-project-new"
        | "admin_admin-projects"
        | "admin_admin-project-detail" => {
            vec!["common.lang", "pagination.lang", "projects.lang"]
        }
        "users_users-index"
        | "users_users-user-detail"
        | "users_users-user-activate" => {
            vec!["common.lang", "pagination.lang", "users.lang"]
        }
        _ => vec![],
    }
}

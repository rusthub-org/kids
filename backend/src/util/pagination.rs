use mongodb::{
    Collection,
    bson::{oid::ObjectId, Document, doc},
    options::{CountOptions, FindOptions},
};
use serde::{Serialize, Deserialize};

use crate::util::constant::CFG;

use crate::{users, projects};

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct PageInfo {
    pub current_stuff: Option<String>,
    pub current_page: Option<u32>,
    pub first_cursor: Option<ObjectId>,
    pub last_cursor: Option<ObjectId>,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct ResCount {
    pub pages_count: Option<u32>,
    pub total_count: Option<u64>,
}

pub async fn count_pages_and_total(
    coll: &Collection<Document>,
    filter_doc: Option<Document>,
    count_opt: Option<CountOptions>,
) -> (u32, u64) {
    let total_count =
        coll.count_documents(filter_doc, count_opt).await.unwrap();

    let page_size = CFG.get("PAGE_SIZE").unwrap().parse::<u64>().unwrap();
    let pages_mod = total_count % page_size;
    let pages_count = match pages_mod {
        0 => total_count / page_size,
        _ => total_count / page_size + 1,
    } as u32;

    (pages_count, total_count)
}

pub async fn calculate_current_filter_skip(
    from_page: u32,
    first_oid: String,
    last_oid: String,
    filter_doc: &mut Document,
) -> (u32, u64) {
    let mut current_page = from_page;
    let mut skip_x = 0;
    if "".ne(&first_oid) && "-".ne(&first_oid) {
        let first_cursor = ObjectId::parse_str(first_oid).unwrap();
        filter_doc.insert("_id", doc! {"$gte": first_cursor});
        current_page = from_page - 1;
        skip_x = (current_page - 1) as u64;
    } else if "".ne(&last_oid) && "-".ne(&last_oid) {
        let last_cursor = ObjectId::parse_str(last_oid).unwrap();
        filter_doc.insert("_id", doc! {"$lte": last_cursor});
        current_page = from_page + 1;
    };

    (current_page, skip_x)
}

pub async fn find_options(
    sort_doc: Option<Document>,
    skip_x: u64,
) -> FindOptions {
    let page_size = CFG.get("PAGE_SIZE").unwrap().parse::<i64>().unwrap();
    let find_options = FindOptions::builder()
        .sort(sort_doc)
        .skip(skip_x * (page_size as u64))
        .limit(page_size)
        .build();

    find_options
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct UsersResult {
    pub page_info: PageInfo,
    pub res_count: ResCount,
    pub current_items: Vec<users::models::User>,
}

#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct ProjectsResult {
    pub page_info: PageInfo,
    pub res_count: ResCount,
    pub current_items: Vec<projects::models::Project>,
}

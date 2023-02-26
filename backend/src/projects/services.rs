use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, DateTime, Document, doc, from_document, to_document,
        from_bson,
    },
    options::FindOptions,
};
use async_graphql::Error;
use chrono::Duration;

use crate::util::{
    constant::GqlResult,
    common::bson_dt_nyr,
    pagination::{
        ProjectsResult, PageInfo, ResCount, count_pages_and_total,
        calculate_current_filter_skip, find_options,
    },
};

use crate::users;
use crate::categories;
use crate::{topics, topics::models::TopicProject};
use super::models::{
    Project, ProjectNew, File, FileNew, ProjectFileNew, ProjectFile,
};

const PROJECTS_STUFF: &str = "projects";

// create new project
pub async fn project_new(
    db: &Database,
    project_new: ProjectNew,
) -> GqlResult<Project> {
    let coll = db.collection::<Document>("projects");

    let now = DateTime::now();
    let now2ago = now.to_chrono() + Duration::days(-2);
    let filter_doc = doc! {
        "user_id": &project_new.user_id,
        "subject": &project_new.subject,
        "created_at": {"$gte": now2ago} // "$lte": now
    };
    let exist_document = coll.find_one(filter_doc, None).await?;

    if exist_document.is_none() {
        let mut new_document = to_document(&project_new)?;
        new_document.insert("created_at", now);
        new_document.insert("updated_at", now);

        let project_res =
            coll.insert_one(new_document, None).await.expect("写入未成功");
        let project_id = from_bson(project_res.inserted_id)?;

        project_by_id(db, project_id).await
    } else {
        let project: Project = from_document(exist_document.unwrap())?;

        Err(Error::new(bson_dt_nyr(project.created_at).await))
    }
}

pub async fn project_by_id(
    db: &Database,
    project_id: ObjectId,
) -> GqlResult<Project> {
    let coll = db.collection::<Document>("projects");

    let project_document = coll
        .find_one(doc! {"_id": project_id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let project: Project = from_document(project_document)?;
    Ok(project)
}

pub async fn project_update_one_field_by_id(
    db: &Database,
    project_id: ObjectId,
    field_name: String,
    field_val: String,
) -> GqlResult<Project> {
    let coll = db.collection::<Document>("projects");

    let query_doc = doc! {"_id": project_id};
    let update_doc = match field_name.as_str() {
        "status" => {
            doc! {"$set": {
                field_name: field_val.parse::<i32>()?,
                "updated_at": DateTime::now()
            }}
        }
        "hits" | "applicants" => {
            doc! {"$inc": {field_name: field_val.parse::<i64>()?}}
        }
        _ => doc! {},
    };

    coll.update_one(query_doc, update_doc, None).await?;

    project_by_id(db, project_id).await
}

// get random project
pub async fn project_random_id(db: &Database) -> GqlResult<ObjectId> {
    let coll = db.collection::<Document>("projects");

    let now = DateTime::now();
    let days_before = now.to_chrono() + Duration::days(-7);
    let filter_doc = doc! {
        "status": {"$gte": 1},
        "updated_at": {"$gte": days_before}
    };

    let match_doc = doc! {"$match": filter_doc};
    let mut cursor = coll
        .aggregate(vec![doc! {"$sample": {"size": 1}}, match_doc], None)
        .await?;

    if let Some(document_res) = cursor.next().await {
        let project: Project = from_document(document_res?)?;
        Ok(project._id)
    } else {
        Err(Error::new("查询未成功"))
    }
}

pub async fn projects(
    db: &Database,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

async fn filter_status(status: i8, filter_doc: &mut Document) {
    if status > 0 {
        filter_doc.insert("status", doc! {"$gte": status as i32});
    } else if status < 0 {
        filter_doc.insert("status", status as i32);
    }
}

pub async fn projects_in_position(
    db: &Database,
    username: String,
    position: String,
    limit: i64,
) -> GqlResult<Vec<Project>> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = users::services::user_by_username(db, username).await?;
        filter_doc.insert("user_id", &user._id);
    }

    match position.trim() {
        "managed" => filter_doc.insert("status", doc! {"$gte": 6}),
        "recommended" => filter_doc.insert("status", doc! {"$gte": 2}),
        "published" => filter_doc.insert("status", doc! {"$gte": 1}),
        _ => None,
    };

    let sort_doc = doc! {"_id": -1};
    let find_options =
        FindOptions::builder().sort(sort_doc).limit(limit).build();
    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    Ok(projects)
}

pub async fn projects_by_user_id(
    db: &Database,
    user_id: ObjectId,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {"user_id": user_id};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

pub async fn projects_by_username(
    db: &Database,
    username: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let user = users::services::user_by_username(db, username).await?;
    projects_by_user_id(db, user._id, from_page, first_oid, last_oid, status)
        .await
}

// Get all projects by category_id
pub async fn projects_by_category_id(
    db: &Database,
    category_id: ObjectId,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {"category_id": category_id};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

// Get all projects by category_slug
pub async fn projects_by_category_slug(
    db: &Database,
    category_slug: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let category =
        categories::services::category_by_slug(db, category_slug).await?;
    projects_by_category_id(
        db,
        category._id,
        from_page,
        first_oid,
        last_oid,
        status,
    )
    .await
}

// Get all projects by topic_id
pub async fn projects_by_topic_id(
    db: &Database,
    topic_id: ObjectId,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let topics_projects = topics_projects_by_topic_id(db, topic_id).await;

    let mut project_ids = vec![];
    for topic_project in topics_projects {
        project_ids.push(topic_project.project_id);
    }
    project_ids.sort();
    project_ids.dedup();

    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {"_id": {"$in": project_ids}};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

// Get all projects by topic_slug
pub async fn projects_by_topic_slug(
    db: &Database,
    topic_slug: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let topic = topics::services::topic_by_slug(db, topic_slug).await?;
    projects_by_topic_id(db, topic._id, from_page, first_oid, last_oid, status)
        .await
}

// get all TopicProject list by topic_id
async fn topics_projects_by_topic_id(
    db: &Database,
    topic_id: ObjectId,
) -> Vec<TopicProject> {
    let coll_topics_projects =
        db.collection::<Document>("topics_users_projects");
    let mut cursor_topics_projects = coll_topics_projects
        .find(
            doc! {
                "topic_id": topic_id,
                "project_id": { "$exists": true }
            },
            None,
        )
        .await
        .unwrap();

    let mut topics_projects: Vec<TopicProject> = vec![];
    while let Some(result) = cursor_topics_projects.next().await {
        match result {
            Ok(document) => {
                let topic_project: TopicProject =
                    from_document(document).unwrap();
                topics_projects.push(topic_project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    topics_projects
}

// Get all projects by investment
pub async fn projects_by_investment(
    db: &Database,
    investment_min: i64,
    investment_max: i64,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc =
        doc! {"investment":  {"$gte": investment_min, "$lte": investment_max}};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

// Get all projects by worker_type
pub async fn projects_by_worker_type(
    db: &Database,
    worker_type: String,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {"worker_type": {"$regex": worker_type}};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

// Get all projects by external
pub async fn projects_by_external(
    db: &Database,
    external: bool,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<ProjectsResult> {
    let coll = db.collection::<Document>("projects");

    let mut filter_doc = doc! {"external": external};
    filter_status(status, &mut filter_doc).await;

    let (pages_count, total_count) =
        count_pages_and_total(&coll, Some(filter_doc.clone()), None).await;
    let (current_page, skip_x) = calculate_current_filter_skip(
        from_page,
        first_oid,
        last_oid,
        &mut filter_doc,
    )
    .await;

    let sort_doc = doc! {"_id": -1};
    let find_options = find_options(Some(sort_doc), skip_x).await;

    let mut cursor = coll.find(filter_doc, find_options).await?;

    let mut projects: Vec<Project> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let project = from_document(document)?;
                projects.push(project);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let projects_result = ProjectsResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(PROJECTS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match projects.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match projects.last() {
                Some(user) => Some(user._id),
                _ => None,
            },
            has_previous_page: current_page > 1,
            has_next_page: current_page < pages_count,
        },
        res_count: ResCount {
            pages_count: Some(pages_count),
            total_count: Some(total_count),
        },
        current_items: projects,
    };

    Ok(projects_result)
}

// Create new file
pub async fn file_new(db: &Database, file_new: FileNew) -> GqlResult<File> {
    let coll = db.collection::<Document>("files");

    let new_document = to_document(&file_new)?;

    let file_res =
        coll.insert_one(new_document, None).await.expect("写入未成功");
    let file_id = from_bson(file_res.inserted_id)?;

    file_by_id(db, file_id).await
}

// get file by id
pub async fn file_by_id(db: &Database, id: ObjectId) -> GqlResult<File> {
    let coll = db.collection::<Document>("files");

    let file_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let file: File = from_document(file_document)?;
    Ok(file)
}

// Create new project_file
pub async fn project_file_new(
    db: &Database,
    project_file_new: ProjectFileNew,
) -> GqlResult<ProjectFile> {
    let coll = db.collection::<Document>("projects_files");

    let exist_document = coll
        .find_one(
            doc! {
            "user_id": &project_file_new.user_id,
            "project_id": &project_file_new.project_id,
            "file_id": &project_file_new.file_id},
            None,
        )
        .await?;
    if exist_document.is_none() {
        let new_document = to_document(&project_file_new)?;
        let project_file_res =
            coll.insert_one(new_document, None).await.expect("写入未成功");
        let project_file_id = from_bson(project_file_res.inserted_id)?;

        project_file_by_id(db, project_file_id).await
    } else {
        Err(Error::new("记录已存在"))
    }
}

// get project_file by its id
async fn project_file_by_id(
    db: &Database,
    id: ObjectId,
) -> GqlResult<ProjectFile> {
    let coll = db.collection::<Document>("projects_files");

    let project_file_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let project_file: ProjectFile = from_document(project_file_document)?;
    Ok(project_file)
}

// get all files of one project by project_id
pub async fn files_by_project_id(
    db: &Database,
    project_id: ObjectId,
) -> GqlResult<Vec<File>> {
    let projects_files = projects_files_by_project_id(db, project_id).await;

    let mut file_ids = vec![];
    for project_file in projects_files {
        file_ids.push(project_file.file_id);
    }

    let filter_doc = doc! {"_id": {"$in": file_ids}};

    let coll = db.collection::<Document>("files");
    let mut cursor = coll.find(filter_doc, None).await?;

    let mut files: Vec<File> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let file = from_document(document)?;
                files.push(file);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    Ok(files)
}

// get all ProjectFile by project_id
async fn projects_files_by_project_id(
    db: &Database,
    project_id: ObjectId,
) -> Vec<ProjectFile> {
    let coll_projects_files = db.collection::<Document>("projects_files");
    let mut cursor_projects_files = coll_projects_files
        .find(doc! {"project_id": project_id}, None)
        .await
        .unwrap();

    let mut projects_files: Vec<ProjectFile> = vec![];
    while let Some(result) = cursor_projects_files.next().await {
        match result {
            Ok(document) => {
                let project_file: ProjectFile =
                    from_document(document).unwrap();
                projects_files.push(project_file);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    projects_files
}

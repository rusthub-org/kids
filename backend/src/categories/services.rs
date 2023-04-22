use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, Document, doc, from_document, to_document, from_bson,
        DateTime,
    },
    options::FindOptions,
};
use async_graphql::Error;

use crate::util::{constant::GqlResult, common::slugify};

use super::models::{Category, CategoryUser, CategoryNew, CategoryUserNew};

// Create new category
pub async fn category_new(
    db: &Database,
    mut category_new: CategoryNew,
) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    category_new.name_zh = category_new.name_zh.trim().to_string();
    let name_check = "".ne(&category_new.name_zh)
        && "-".ne(&category_new.name_zh)
        && "".ne(&category_new.name_en)
        && "-".ne(&category_new.name_en);
    match name_check {
        true => {
            let exist_document = coll
                .find_one(
                    doc! {
                        "name_zh": &category_new.name_zh,
                        "name_en": &category_new.name_en
                    },
                    None,
                )
                .await?;

            if exist_document.is_none() {
                let slug_zh = slugify(&category_new.name_zh).await;
                let slug_en = slugify(&category_new.name_en).await;
                let slug_ms = DateTime::now().timestamp_millis();
                if slug_zh == slug_en {
                    category_new.slug = format!("{}-{}", slug_zh, slug_ms);
                } else {
                    category_new.slug =
                        format!("{}-{}-{}", slug_zh, slug_en, slug_ms);
                }

                let mut new_document = to_document(&category_new)?;
                let now = DateTime::now();
                new_document.insert("created_at", now);
                new_document.insert("updated_at", now);

                let category_res = coll
                    .insert_one(new_document, None)
                    .await
                    .expect("写入未成功");
                let category_id = from_bson(category_res.inserted_id)?;

                category_by_id(db, category_id).await
            } else {
                let category: Category =
                    from_document(exist_document.unwrap())?;

                Err(Error::new(format!(
                    "{}（中）| {}（英），此类别已创建",
                    category.name_zh, category.name_en
                )))
            }
        }
        _ => Err(Error::new("名称不合法")),
    }
}

// Create new category_user
pub async fn category_user_new(
    db: &Database,
    category_user_new: CategoryUserNew,
) -> GqlResult<CategoryUser> {
    let coll = db.collection::<Document>("categories_users");

    let exist_document = coll
        .find_one(
            doc! {
            "user_id": &category_user_new.user_id,
            "category_id": &category_user_new.category_id},
            None,
        )
        .await?;

    if exist_document.is_none() {
        let new_document = to_document(&category_user_new)?;
        let category_user_res =
            coll.insert_one(new_document, None).await.expect("写入未成功");
        let category_user_id = from_bson(category_user_res.inserted_id)?;

        category_user_by_id(db, category_user_id).await
    } else {
        Err(Error::new("记录已存在"))
    }
}

// get category_user by its id
async fn category_user_by_id(
    db: &Database,
    id: ObjectId,
) -> GqlResult<CategoryUser> {
    let coll = db.collection::<Document>("categories_users");

    let category_user_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let category_user: CategoryUser = from_document(category_user_document)?;
    Ok(category_user)
}

// get all categories
pub async fn categories(db: &Database) -> GqlResult<Vec<Category>> {
    let coll = db.collection::<Document>("categories");

    let find_options = FindOptions::builder().sort(doc! {"quotes": -1}).build();
    let mut cursor = coll.find(None, find_options).await?;

    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by user_id
pub async fn categories_by_user_id(
    db: &Database,
    user_id: ObjectId,
) -> GqlResult<Vec<Category>> {
    let categories_users = categories_users_by_user_id(db, user_id).await;

    let mut category_ids: Vec<ObjectId> = vec![];
    for category_user in categories_users {
        category_ids.push(category_user.category_id);
    }

    let coll_categories = db.collection::<Document>("categories");
    let mut cursor_categories =
        coll_categories.find(doc! {"_id": {"$in": category_ids}}, None).await?;

    let mut categories: Vec<Category> = vec![];
    while let Some(result) = cursor_categories.next().await {
        match result {
            Ok(document) => {
                let category: Category = from_document(document)?;
                categories.push(category);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    Ok(categories)
}

// get all categories by username
pub async fn categories_by_username(
    db: &Database,
    username: String,
) -> GqlResult<Vec<Category>> {
    let user = crate::users::services::user_by_username(db, username).await?;
    categories_by_user_id(db, user._id).await
}

// get category by its id
pub async fn category_by_id(
    db: &Database,
    id: ObjectId,
) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get category by its slug
pub async fn category_by_slug(
    db: &Database,
    slug: String,
) -> GqlResult<Category> {
    let coll = db.collection::<Document>("categories");

    let category_document = coll
        .find_one(doc! {"slug": slug.to_lowercase()}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let category: Category = from_document(category_document)?;
    Ok(category)
}

// get all CategoryUser list by user_id
async fn categories_users_by_user_id(
    db: &Database,
    user_id: ObjectId,
) -> Vec<CategoryUser> {
    let coll_categories_users = db.collection::<Document>("categories_users");
    let mut cursor_categories_users = coll_categories_users
        .find(doc! {"user_id": user_id}, None)
        .await
        .unwrap();

    let mut categories_users: Vec<CategoryUser> = vec![];
    while let Some(result) = cursor_categories_users.next().await {
        match result {
            Ok(document) => {
                let category_user: CategoryUser =
                    from_document(document).unwrap();
                categories_users.push(category_user);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    categories_users
}

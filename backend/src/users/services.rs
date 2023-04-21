use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{
        oid::ObjectId, DateTime, Document, doc, from_document, to_document,
        from_bson,
    },
};
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use regex::Regex;

use crate::util::{
    constant::{CFG, GqlResult},
    cred::{cred_encode, cred_verify, Claims, token_data},
    pagination::{
        UsersResult, PageInfo, ResCount, count_pages_and_total,
        calculate_current_filter_skip, find_options,
    },
};

use super::models::{User, UserNew, SignInfo, Wish, WishNew};

const USERS_STUFF: &str = "users";

pub async fn user_register(
    db: &Database,
    mut user_new: UserNew,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    user_new.email = user_new.email.trim().to_lowercase();
    user_new.username = user_new.username.trim().to_lowercase();

    if user_by_email(db, user_new.email.to_owned()).await.is_err()
        && user_by_username(db, user_new.username.to_owned()).await.is_err()
    {
        user_new.cred = cred_encode(&user_new.username, &user_new.cred).await;

        let mut new_document = to_document(&user_new)?;
        let now = DateTime::now();
        new_document.insert("created_at", now);
        new_document.insert("updated_at", now);

        let user_res =
            coll.insert_one(new_document, None).await.expect("写入未成功");
        let user_id = from_bson(user_res.inserted_id)?;

        user_by_id(db, user_id).await
    } else {
        Err(Error::new("register-failed-username-email-exists"))
    }
}

pub async fn user_sign_in(
    db: &Database,
    signature: String,
    password: String,
) -> GqlResult<SignInfo> {
    let signature = &signature.to_lowercase();

    let user_res;
    let is_email = Regex::new(r"(@)")?.is_match(signature);
    if is_email {
        user_res = user_by_email(db, signature.to_owned()).await;
    } else {
        user_res = user_by_username(db, signature.to_owned()).await;
    }

    if let Ok(user) = user_res {
        match user.status {
            1..=10 => {
                let is_verified =
                    cred_verify(&user.username, &password, &user.cred).await;
                if is_verified {
                    let site_kid = CFG.get("SITE_KID").unwrap();
                    let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();
                    let claim_exp =
                        CFG.get("CLAIM_EXP").unwrap().parse::<usize>()?;

                    let mut header = Header::default();
                    header.kid = Some(String::from(site_kid));
                    header.alg = Algorithm::HS512;

                    let claims = Claims {
                        email: user.email,
                        username: user.username.clone(),
                        exp: claim_exp,
                    };

                    let mut sign_info = SignInfo {
                        username: user.username,
                        token: String::from("无令牌！"),
                    };
                    sign_info.token = encode(
                        &header,
                        &claims,
                        &EncodingKey::from_secret(site_key),
                    )?;

                    Ok(sign_info)
                } else {
                    Err(Error::new("sign-in-incorrect"))
                }
            }
            0 => Err(Error::new("sign-in-not-activation")
                .extend_with(|_, e| e.set("user_id", user._id.to_string()))),
            -1 => Err(Error::new("sign-in-banned")),
            _ => Err(Error::new("sign-in-security-problem")),
        }
    } else {
        Err(Error::new("sign-in-not-registration"))
    }
}

// get user info by id
pub async fn user_by_id(db: &Database, id: ObjectId) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let user_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("账户不存在")
        .unwrap();

    let user: User = from_document(user_document)?;
    Ok(user)
}

pub async fn user_update_one_field_by_id(
    db: &Database,
    user_id: ObjectId,
    field_name: String,
    field_val: String,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let query_doc = doc! {"_id": user_id};
    let update_doc = match field_name.as_str() {
        "status" => {
            doc! {"$set": {field_name: field_val.parse::<i32>()?}}
        }
        _ => doc! {},
    };

    coll.update_one(query_doc, update_doc, None).await?;

    user_by_id(db, user_id).await
}

// get user info by email
pub async fn user_by_email(db: &Database, email: String) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let user_document = coll.find_one(doc! {"email": &email}, None).await?;
    if user_document.is_some() {
        let user: User = from_document(user_document.unwrap())?;
        Ok(user)
    } else {
        Err(Error::new(format!("{} - 未注册", email)))
    }
}

// get user info by username
pub async fn user_by_username(
    db: &Database,
    username: String,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let user_document =
        coll.find_one(doc! {"username": &username}, None).await?;
    if user_document.is_some() {
        let user: User = from_document(user_document.unwrap())?;
        Ok(user)
    } else {
        Err(Error::new(format!("{} - 未注册", username)))
    }
}

// Change user password
pub async fn user_change_password(
    db: &Database,
    pwd_cur: String,
    pwd_new: String,
    token: String,
) -> GqlResult<User> {
    let token_data = token_data(&token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = user_by_email(db, email).await;
        if let Ok(mut user) = user_res {
            if cred_verify(&user.username, &pwd_cur, &user.cred).await {
                user.cred = cred_encode(&user.username, &pwd_new).await;

                let coll = db.collection::<Document>("users");
                coll.update_one(
                    doc! {"_id": &user._id},
                    doc! {"$set": {"cred": &user.cred}},
                    None,
                )
                .await
                .expect("更新未成功");

                Ok(user)
            } else {
                Err(Error::new("密码验证失败"))
            }
        } else {
            Err(Error::new("账户未注册"))
        }
    } else {
        Err(Error::new("令牌验证失败"))
    }
}

// update user profile
pub async fn user_update_profile(
    db: &Database,
    user_new: UserNew,
    token: String,
) -> GqlResult<User> {
    let token_data = token_data(&token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = user_by_email(db, email).await;
        if let Ok(mut user) = user_res {
            let coll = db.collection::<Document>("users");

            user.email = user_new.email.to_lowercase();
            user.username = user_new.username.to_lowercase();

            let user_document = to_document(&user)?;

            coll.find_one_and_replace(
                doc! {"_id": &user._id},
                user_document,
                None,
            )
            .await
            .expect("更新未成功");

            Ok(user)
        } else {
            Err(Error::new("账户未注册"))
        }
    } else {
        Err(Error::new("令牌验证失败"))
    }
}

// Get all Users
pub async fn users(
    db: &Database,
    from_page: u32,
    first_oid: String,
    last_oid: String,
    status: i8,
) -> GqlResult<UsersResult> {
    let coll = db.collection::<Document>("users");

    let mut filter_doc = doc! {
        "status": {
            "$gte": status as i32,
            "$lte": 12
        }
    };

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

    let mut users: Vec<User> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let user = from_document(document)?;
                users.push(user);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    let users_result = UsersResult {
        page_info: PageInfo {
            current_stuff: Some(String::from(USERS_STUFF)),
            current_page: Some(current_page),
            first_cursor: match users.first() {
                Some(user) => Some(user._id),
                _ => None,
            },
            last_cursor: match users.last() {
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
        current_items: users,
    };

    Ok(users_result)
}

// Create new wish
pub async fn wish_new(db: &Database, wish_new: WishNew) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");

    let exist_document = coll
        .find_one(
            doc! {"user_id": &wish_new.user_id, "aphorism": &wish_new.aphorism},
            None,
        )
        .await?;

    if exist_document.is_none() {
        let mut new_document = to_document(&wish_new)?;
        let now = DateTime::now();
        new_document.insert("created_at", now);
        new_document.insert("updated_at", now);

        let wish_res =
            coll.insert_one(new_document, None).await.expect("写入未成功");
        let wish_id = from_bson(wish_res.inserted_id)?;

        wish_by_id(db, wish_id).await
    } else {
        Err(Error::new("记录已存在"))
    }
}

// get wish by its id
async fn wish_by_id(db: &Database, id: ObjectId) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");

    let wish_document = coll
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("查询未成功")
        .unwrap();

    let wish: Wish = from_document(wish_document)?;
    Ok(wish)
}

// get all wishes
pub async fn wishes(db: &Database, published: i8) -> GqlResult<Vec<Wish>> {
    let mut filter_doc = doc! {};
    if published > 0 {
        filter_doc.insert("published", true);
    } else if published < 0 {
        filter_doc.insert("published", false);
    }
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll.find(filter_doc, None).await?;

    let mut wishes: Vec<Wish> = vec![];
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let wish = from_document(document)?;
                wishes.push(wish);
            }
            Err(error) => {
                println!("\n\n\n{}\n\n\n", error);
            }
        }
    }

    Ok(wishes)
}

// get random wish
pub async fn wish_random(db: &Database, username: String) -> GqlResult<Wish> {
    let mut filter_doc = doc! {"published": true};
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        let user = user_by_username(db, username).await?;
        filter_doc.insert("user_id", &user._id);
    }
    let match_doc = doc! {"$match": filter_doc};

    let wish_one_res = wish_one(db, match_doc).await;
    if wish_one_res.is_ok() {
        wish_one_res
    } else {
        wish_one(db, doc! {"$match": {"published": true}}).await
    }
}

async fn wish_one(db: &Database, match_doc: Document) -> GqlResult<Wish> {
    let coll = db.collection::<Document>("wishes");
    let mut cursor = coll
        .aggregate(vec![doc! {"$sample": {"size": 1}}, match_doc], None)
        .await?;

    if let Some(document_res) = cursor.next().await {
        let wish = from_document(document_res?)?;
        Ok(wish)
    } else {
        Err(Error::new("查询未成功"))
    }
}

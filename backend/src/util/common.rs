// Generate friendly slug from the given string
pub async fn slugify(str: &str) -> String {
    use deunicode::deunicode_with_tofu;

    let slug = deunicode_with_tofu(str.trim(), "-")
        .to_lowercase()
        .replace(" ", "-")
        .replace("[", "-")
        .replace("]", "-")
        .replace("\"", "-")
        .replace("/", "-")
        .replace("?", "-")
        .replace("&", "-")
        .replace(".", "-")
        .replace("#", "++++")
        .replace("---", "-")
        .replace("--", "-");

    slug
}

// bson::DateTime -> Y-M-D
pub async fn bson_dt_nyr(dt: mongodb::bson::DateTime) -> String {
    dt.to_chrono()
        .with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
        .format(crate::util::constant::DTF_YMD)
        .to_string()
}

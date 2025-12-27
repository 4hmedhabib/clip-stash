use super::model;
use crate::data::{DataError, DatabasePool};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();

    let clip = sqlx::query_as::<_, model::Clip>("SELECT * FROM clips WHERE shortcode = ?")
        .bind(shortcode)
        .fetch_one(pool)
        .await?;

    Ok(clip)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();

    let _ = sqlx::query!(
        r#"INSERT INTO clips (
        clip_id,
        shortcode,
        content,
        title,
        posted,
        expires,
        password,
        hits)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0_i64,
    )
    .execute(pool)
    .await?;

    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model = model.into();

    let _ = sqlx::query!(
        r#"UPDATE clips SET  
        content = ?,
        title = ?,
        expires = ?,
        password = ?
        WHERE shortcode = ?"#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode,
    )
    .execute(pool)
    .await?;

    get_clip(model.shortcode, pool).await
}

use crate::database::{DbPool, get_db_conn};
use crate::models::media::{Media, NewMedia};
use crate::schema::media::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use crate::storage::S3Storage;

/// Uploads a media file record to the database
pub fn upload_media_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
    s3_key_val: &str,
    file_name_val: &str,
    file_type_val: &str,
) -> Result<Media, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let new_media = NewMedia {
        id: Uuid::new_v4(),
        user_id: *user_id_val,
        s3_key: s3_key_val.to_string(),
        file_name: file_name_val.to_string(),
        file_type: file_type_val.to_string(),
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(media)
        .values(&new_media)
        .get_result(&mut conn)
}

/// Deletes all media for a user from S3 and DB
pub async fn delete_media_by_user_id(
    pool: &DbPool,
    user_id_val: &Uuid,
    s3_client: &S3Storage,
) -> Result<(), actix_web::Error> {
    let mut conn = get_db_conn(pool)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let media_list = media
        .filter(user_id.eq(user_id_val))
        .load::<Media>(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    for m in media_list {
        s3_client.delete_file(&m.s3_key).await?;
    }

    diesel::delete(media.filter(user_id.eq(user_id_val)))
        .execute(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(())
}
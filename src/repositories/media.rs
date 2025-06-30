use crate::database::{DbPool, get_db_conn};
use crate::models::media::{Media, NewMedia};
use crate::schema::media::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

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
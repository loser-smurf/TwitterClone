use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::media;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[diesel(table_name = media)]
pub struct Media {
    pub id: Uuid,
    pub user_id: Uuid,
    pub s3_key: String,
    pub file_name: String,
    pub file_type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = media)]
pub struct NewMedia {
    pub id: Uuid,
    pub user_id: Uuid,
    pub s3_key: String,
    pub file_name: String,
    pub file_type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadMediaForm {
    pub media_url: String,
} 
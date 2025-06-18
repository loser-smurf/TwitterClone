use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub username: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}


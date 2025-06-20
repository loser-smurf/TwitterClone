use crate::models::users::User;
use crate::schema::follows;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = follows)]
#[diesel(primary_key(follower_id, followed_id))]
pub struct Follow {
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = follows)]
pub struct NewFollow {
    pub follower_id: Uuid,
    pub followed_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowWithUsers {
    #[serde(flatten)]
    pub follow: Follow,
    pub follower: User,
    pub followed: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub user: User,
    pub followers_count: i64,
    pub following_count: i64,
    pub tweets_count: i64,
    pub is_following: bool,
}

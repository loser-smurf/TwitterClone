use crate::models::tweets::Tweet;
use crate::models::users::User;
use crate::schema::likes;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tweet))]
#[diesel(table_name = likes)]
pub struct Like {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tweet_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = likes)]
pub struct NewLike {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tweet_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeWithUser {
    #[serde(flatten)]
    pub like: Like,
    pub user: User,
}

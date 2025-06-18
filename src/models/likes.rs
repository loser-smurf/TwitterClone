use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable, Associations};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::likes;
use crate::models::users::User;
use crate::models::tweets::Tweet;

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

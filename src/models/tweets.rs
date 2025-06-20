use crate::models::users::User;
use crate::schema::tweets;
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tweets)]
pub struct Tweet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<Option<String>>>,
    pub reply_to_id: Option<Uuid>,
    pub is_retweet: bool,
    pub original_tweet_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tweets)]
pub struct NewTweet {
    pub user_id: Uuid,
    pub content: String,
    pub media_urls: Option<Vec<Option<String>>>,
    pub reply_to_id: Option<Uuid>,
    pub is_retweet: bool,
    pub original_tweet_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetWithUser {
    #[serde(flatten)]
    pub tweet: Tweet,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetWithStats {
    #[serde(flatten)]
    pub tweet: Tweet,
    pub user: User,
    pub likes_count: i64,
    pub retweets_count: i64,
    pub replies_count: i64,
    pub is_liked_by_current_user: bool,
    pub is_retweeted_by_current_user: bool,
}

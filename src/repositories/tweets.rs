use crate::database::{DbPool, get_db_conn};
use crate::models::tweets::{NewTweet, Tweet};
use crate::schema::tweets::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Create a tweet
pub fn create_tweet_repo(
    pool: &DbPool,
    user_id_val: &Uuid,
    content_val: &str,
) -> Result<Tweet, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let new_tweet = NewTweet {
        user_id: *user_id_val,
        content: content_val.to_string(),
        media_urls: None,
        reply_to_id: None,
        is_retweet: false,
        original_tweet_id: None,
    };

    let tweet = diesel::insert_into(tweets)
        .values(&new_tweet)
        .get_result::<Tweet>(&mut conn)?;

    Ok(tweet)
}

/// Gest a tweet
pub fn get_tweet_repo(pool: &DbPool, tweet_id_val: &Uuid) -> Result<Tweet, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let tweet = tweets.find(tweet_id_val).first::<Tweet>(&mut conn)?;

    Ok(tweet)
}

/// Gets paginated list of tweets
pub fn get_tweets_repo(
    pool: &DbPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Tweet>, i64), diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let offset = (page - 1) * per_page;

    let total_count = tweets.count().get_result(&mut conn)?;

    let tweets_list = tweets
        .order(created_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Tweet>(&mut conn)?;

    Ok((tweets_list, total_count))
}

/// Deletes a tweet
pub fn delete_tweet_repo(
    pool: &DbPool,
    tweet_id_val: &Uuid,
) -> Result<bool, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    diesel::delete(tweets.filter(id.eq(tweet_id_val))).execute(&mut conn)?;
    Ok(true)
}

/// Creates a reply
pub fn create_reply_repo(
    pool: &DbPool,
    tweet_id_val: &Uuid,
    user_id_val: &Uuid,
    content_val: &str,
) -> Result<Tweet, diesel::result::Error> {
    let mut conn = get_db_conn(pool)?;

    let new_reply = NewTweet {
        user_id: *user_id_val,
        content: content_val.to_string(),
        media_urls: None,
        reply_to_id: Some(*tweet_id_val),
        is_retweet: false,
        original_tweet_id: None,
    };

    let reply = diesel::insert_into(tweets)
        .values(&new_reply)
        .get_result::<Tweet>(&mut conn)?;

    Ok(reply)
}
